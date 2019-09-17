import ts = require('typescript');
import { isStructType, propertiesOfType } from '../jsii/jsii-utils';
import { NO_SYNTAX, OTree, renderTree } from "../o-tree";
import { containsNewline, matchAst, nodeOfType, preserveSeparatingNewlines, stripCommentMarkers } from '../typescript/ast-utils';
import { ImportStatement } from '../typescript/imports';
import { startsWithUppercase } from "../util";
import { AstContext, DefaultVisitor, nimpl } from "../visitor";

interface StructVar {
  variableName: string;
  type: ts.Type | undefined;
}

type ReturnFromTree<A> = { value?: A; };

interface PythonLanguageContext {
  /**
   * Whether we're currently rendering a parameter in tail position
   *
   * If so, and the parameter is of type struct, explode it to keyword args
   * and return its information in `returnExplodedParameter`.
   */
  readonly tailPositionParameter?: boolean;

  /**
   * Used to return details about any exploded parameter
   */
  readonly returnExplodedParameter?: ReturnFromTree<StructVar>;

  /**
   * Whether we're currently rendering a value/expression in tail position
   *
   * If so, and the expression seems to be of a struct type, explode it
   * to keyword args.
   */
  readonly tailPositionArgument?: boolean;

  /**
   * Whether object literal members should render themselves as dict
   * members or keyword args
   */
  readonly renderObjectLiteralAsKeywords?: boolean;

  /**
   * In a code block, if any parameter is exploded, information about the parameter here
   */
  readonly explodedParameter?: StructVar;

  /**
   * Whether we're rendering a method or property inside a class
   */
  readonly inClass?: boolean;

  /**
   * If we're in a method, what is it's name
   *
   * (Used to render super() call.);
   */
  readonly currentMethodName?: string;
}

type PythonVisitorContext = AstContext<PythonLanguageContext>;

export class PythonVisitor extends DefaultVisitor<PythonLanguageContext> {
  public readonly defaultContext = {};

  public mergeContext(old: PythonLanguageContext, update: PythonLanguageContext) {
    return Object.assign({}, old, update);
  }

  public commentRange(node: ts.CommentRange, context: PythonVisitorContext): OTree {
    const commentText = stripCommentMarkers(context.textAt(node.pos, node.end), node.kind === ts.SyntaxKind.MultiLineCommentTrivia);

    return new OTree([...commentText.split('\n').map(l => `# ${l}\n`)], [], {
      // Make sure comment is rendered exactly once in the output tree, no
      // matter how many source nodes it is attached to.
      renderOnce: `comment-${node.pos}`
    });
  }

  public importStatement(node: ImportStatement, context: PythonVisitorContext): OTree {
    const moduleName = this.convertModuleReference(node.packageName);
    if (node.imports.import === 'full') {
      return new OTree([`import ${moduleName} as ${mangleIdentifier(node.imports.alias)}`], [], {
        newline: true,
        attachComment: true
      });
    }
    if (node.imports.import === 'selective') {
      const imports = node.imports.elements.map(im =>
          im.alias
          ? `${mangleIdentifier(im.sourceName)} as ${mangleIdentifier(im.alias)}`
          : mangleIdentifier(im.sourceName));

      return new OTree([`from ${moduleName} import ${imports.join(', ')}`], [], {
        newline: true,
        attachComment: true
      });
    }

    return nimpl(node.node, context);
  }

  public token<A extends ts.SyntaxKind>(node: ts.Token<A>, context: PythonVisitorContext): OTree {
    const text = context.textOf(node);
    const mapped = TOKEN_REWRITES[text];
    if (mapped) { return new OTree([mapped]); }
    return super.token(node, context);
  }

  public identifier(node: ts.Identifier, _context: PythonVisitorContext) {
    const originalIdentifier = node.text;
    return new OTree([mangleIdentifier(originalIdentifier)]);
  }

  public functionDeclaration(node: ts.FunctionDeclaration, context: PythonVisitorContext): OTree {
    return this.functionLike(node, context);
  }

  public constructorDeclaration(node: ts.ConstructorDeclaration, context: PythonVisitorContext): OTree {
    return this.functionLike(node, context, { isConstructor: true });
  }

  public methodDeclaration(node: ts.MethodDeclaration, context: PythonVisitorContext): OTree {
    return this.functionLike(node, context);
  }

  public expressionStatement(node: ts.ExpressionStatement, context: PythonVisitorContext): OTree {
    const text = context.textOf(node);
    if (text === 'true') { return new OTree(['True']); }
    if (text === 'false') { return new OTree(['False']); }

    return super.expressionStatement(node, context);
  }

  // tslint:disable-next-line:max-line-length
  public functionLike(node: ts.FunctionLikeDeclarationBase, context: PythonVisitorContext, opts: { isConstructor?: boolean } = {}): OTree {
    const methodName = opts.isConstructor ? '__init__' : renderTree(context.convert(node.name));

    const [paramDecls, explodedParameter] = this.convertFunctionCallParameters(node.parameters, context);

    const ret = new OTree([
      'def ',
      methodName,
      '(',
      new OTree([], [
        context.currentContext.inClass ? 'self' : undefined,
        ...paramDecls,
      ], {
        separator: ', ',
      }),
      '): ',
    ], [context.convert(node.body, { explodedParameter, currentMethodName: methodName })], {
      suffix: '\n\n',
      attachComment: true
    });

    return ret;
  }

  public block(node: ts.Block, context: PythonVisitorContext): OTree {
    const children = node.statements.length > 0
        ? context.convertAll(node.statements)
        : [new OTree(['pass'])];

    return new OTree([], children, {
      newline: true,
      indent: 4,
      separator: '\n',
      attachComment: true
    });
  }

  public callExpression(node: ts.CallExpression, context: PythonVisitorContext): OTree {
    let expressionText: OTree | string = context.convert(node.expression);

    if (matchAst(node.expression, nodeOfType(ts.SyntaxKind.SuperKeyword)) && context.currentContext.currentMethodName) {
      expressionText = 'super().' + context.currentContext.currentMethodName;
    }

    return new OTree([
      expressionText,
      '(',
      this.convertFunctionCallArguments(node.arguments, context),
      ')'], [], { attachComment: true });
  }

  public propertyAccessExpression(node: ts.PropertyAccessExpression, context: PythonVisitorContext) {
    const fullText = context.textOf(node);
    if (fullText in BUILTIN_FUNCTIONS) {
      return new OTree([BUILTIN_FUNCTIONS[fullText]]);
    }

    const explodedParameter = context.currentContext.explodedParameter;

    // We might be in a context where we've exploded this struct into arguments,
    // in which case we will return just the accessed variable.
    if (explodedParameter && context.textOf(node.expression) === explodedParameter.variableName) {
      return context.convert(node.name);
    }

    return super.propertyAccessExpression(node, context);
  }

  public parameterDeclaration(node: ts.ParameterDeclaration, context: PythonVisitorContext): OTree {
    if (context.currentContext.tailPositionParameter && node.type) {
      const type = context.typeOfType(node.type);
      if (isStructType(type)) {
        // Return the parameter that we exploded so that we can use this information
        // while translating the body.
        if (context.currentContext.returnExplodedParameter) {
          context.currentContext.returnExplodedParameter.value = {
            variableName: context.textOf(node.name),
            type,
          };
        }

        // Explode to fields
        return new OTree([], ['*', ...propertiesOfType(type)], { separator: ', ' });
      }
    }

    return new OTree([context.convert(node.name)]);
  }

  public ifStatement(node: ts.IfStatement, context: PythonVisitorContext): OTree {
    const ifStmt = new OTree(
      ['if ', context.convert(node.expression), ': '],
      [context.convert(node.thenStatement)], { attachComment: true });
    const elseStmt = node.elseStatement ? new OTree([`else: `], [context.convert(node.elseStatement)], { attachComment: true }) : undefined;

    return elseStmt ? new OTree([], [ifStmt, elseStmt], {
      separator: '\n',
      attachComment: true
    }) : ifStmt;
  }

  public objectLiteralExpression(node: ts.ObjectLiteralExpression, context: PythonVisitorContext): OTree {
    if (context.currentContext.tailPositionArgument) {
      // Explode into parent call site
      return new OTree([], [
        preserveNewlines(context.convertAll(node.properties, { renderObjectLiteralAsKeywords: true }), node.properties, context, node, true)],
        {
          separator: ', ',
          indent: 4,
          attachComment: true,
        });
    }

    let prefix = '{';
    let suffix = '}';
    let isStruct = false;

    // If the type of the object literal is a struct, use a class constructor
    const type = context.typeOfExpression(node);
    if (type && isStructType(type)) {
      prefix = type.symbol.name + '(';
      suffix = ')';
      isStruct = true;
    }

    return new OTree([prefix],
      [preserveNewlines(context.convertAll(node.properties, { renderObjectLiteralAsKeywords: isStruct }), node.properties, context, node, true)],
      {
        separator: ', ',
        indent: 4,
        suffix,
        attachComment: true,
      },
    );
  }

  public propertyAssignment(node: ts.PropertyAssignment, context: PythonVisitorContext): OTree {
    let before = '"';
    let mid = '": ';

    if (context.currentContext.renderObjectLiteralAsKeywords) {
      before = '';
      mid = '=';
    }

    return new OTree([
      before,
      context.convert(node.name),
      mid,
      context.convert(node.initializer, {})
    ], [], { attachComment: true });
  }

  public shorthandPropertyAssignment(node: ts.ShorthandPropertyAssignment, context: PythonVisitorContext): OTree {
    let before = '"';
    let mid = '": ';

    if (context.currentContext.renderObjectLiteralAsKeywords) {
      before = '';
      mid = '=';
    }

    return new OTree([
      before,
      context.convert(node.name),
      mid,
      context.convert(node.name)
    ], [], { attachComment: true });
  }

  public newExpression(node: ts.NewExpression, context: PythonVisitorContext): OTree {
    return new OTree([
      context.convert(node.expression),
      '(',
      this.convertFunctionCallArguments(node.arguments, context),
      ')'
    ], [], { attachComment: true });
  }

  public variableDeclaration(node: ts.VariableDeclaration, context: PythonVisitorContext): OTree {
    return new OTree([
      context.convert(node.name),
      ' = ',
      context.convert(node.initializer)
    ], [], { attachComment: true });
  }

  public thisKeyword() {
    return new OTree(['self']);
  }

  public forOfStatement(node: ts.ForOfStatement, context: PythonVisitorContext): OTree {
    // This is what a "for (const x of ...)" looks like in the AST
    let variableName = '???';

    matchAst(node.initializer,
      nodeOfType(ts.SyntaxKind.VariableDeclarationList,
        nodeOfType('var', ts.SyntaxKind.VariableDeclaration)),
      bindings => {
        variableName = mangleIdentifier(context.textOf(bindings.var.name));
      });

    return new OTree([
      'for ',
      variableName,
      ' in ',
      context.convert(node.expression),
      ': '
    ], [context.convert(node.statement)], { attachComment: true });
  }

  public classDeclaration(node: ts.ClassDeclaration, context: PythonVisitorContext): OTree {
    const heritage = flat(Array.from(node.heritageClauses || []).map(h => Array.from(h.types))).map(t => context.convert(t.expression));
    const hasHeritage = heritage.length > 0;

    const members = context.convertAll(node.members, { inClass: true });
    if (members.length === 0) {
      members.push(new OTree(['pass']));
    }

    return new OTree([
      'class ',
      node.name ? context.textOf(node.name) : '???',
      hasHeritage ? '(' : '',
      ...heritage,
      hasHeritage ? ')' : '',
      ': ',
    ], members, {
      separator: '\n\n',
      newline: true,
      indent: 4,
      suffix: '\n\n',
      attachComment: true
    });
  }

  public propertyDeclaration(_node: ts.PropertyDeclaration, _context: PythonVisitorContext): OTree {
    return new OTree([]);
  }

  /**
   * We have to do something special here
   *
   * Best-effort, we remember the fields of struct interfaces and keep track of
   * them. Fortunately we can determine from the name whether what to do.
   */
  public interfaceDeclaration(_node: ts.InterfaceDeclaration, _context: PythonVisitorContext): OTree {
    // Whatever we do, nothing here will have a representation
    return NO_SYNTAX;
  }

  public propertySignature(_node: ts.PropertySignature, _context: PythonVisitorContext): OTree {
    // Does not represent in Python
    return NO_SYNTAX;
  }

  protected convertModuleReference(ref: string) {
    return ref.replace(/^@/, '').replace(/\//g, '.').replace(/-/g, '_');
  }

  /**
   * Convert parameters
   *
   * If the last one has the type of a known struct, explode to keyword-only arguments.
   *
   * Returns a pair of [decls, excploded-var-name].
   */
  // tslint:disable-next-line:max-line-length
  private convertFunctionCallParameters(params: ts.NodeArray<ts.ParameterDeclaration> | undefined, context: PythonVisitorContext): [Array<string | OTree>, StructVar | undefined] {
    if (!params || params.length === 0) { return [[], undefined]; }

    const returnExplodedParameter: ReturnFromTree<StructVar> = {};

    // Convert the last element differently
    const converted: Array<string | OTree> = params.length > 0 ? [
      ...context.convertAll(params.slice(0, params.length - 1)),
      context.convert(last(params), {
        tailPositionParameter: true,
        returnExplodedParameter
      })
    ] : [];

    return [ converted, returnExplodedParameter.value ];
  }

  /**
   * Convert arguments.
   *
   * If the last argument:
   *
   * - is an object literal, explode it.
   * - is itself an exploded argument in our call signature, explode the fields
   */
  private convertFunctionCallArguments(args: ts.NodeArray<ts.Expression> | undefined, context: PythonVisitorContext) {
    if (!args) { return NO_SYNTAX; }
    const converted: Array<string | OTree> = args.length > 0 ? [
      ...context.convertAll(args.slice(0, args.length - 1)),
      context.convert(last(args), {
        tailPositionArgument: true,
      })
    ] : [];

    if (args.length > 0) {
      const lastArg = args[args.length - 1];
      if (ts.isObjectLiteralExpression(lastArg)) {
        // Object literal, render as keyword arguments
        converted.pop();

        // tslint:disable-next-line:max-line-length
        const precedingArg = args.length > 1 ? args[args.length - 2] : undefined;

        // tslint:disable-next-line:max-line-length
        converted.push(preserveNewlines(lastArg.properties.map(p => context.attachComments(p,  convertProp(p))), lastArg.properties, context, precedingArg));
      }

      const explodedParameter = context.currentContext.explodedParameter;

      if (explodedParameter && explodedParameter.type && ts.isIdentifier(lastArg) && lastArg.text === explodedParameter.variableName) {
        // Exploded struct, render fields as keyword arguments
        converted.pop();
        converted.push(new OTree([], propertiesOfType(explodedParameter.type).map(name => new OTree([name, '=', name])), { separator: ', ' }));
      }
    }

    return new OTree([], [preserveNewlines(converted, args, context)], { separator: ', ', indent: 4 });

    function convertProp(prop: ts.ObjectLiteralElementLike) {
      if (ts.isPropertyAssignment(prop)) {
        return new OTree([context.convert(prop.name), '=', context.convert(prop.initializer)]);
      } else if (ts.isShorthandPropertyAssignment(prop)) {
        return new OTree([context.convert(prop.name), '=', context.convert(prop.name)]);
      } else {
        return new OTree(['???']);
      }
    }
  }
}

/**
 * Try to preserve newlines in a converted element tree
 */
// tslint:disable-next-line:max-line-length
function preserveNewlines(elements: Array<OTree | string>, nodes: ReadonlyArray<ts.Node>, context: PythonVisitorContext, leading?: ts.Node, fromStart?: boolean) {
  // tslint:disable-next-line:max-line-length
  const leadingNewline = leading && nodes.length > 0 && containsNewline((fromStart ? context.textFromTo : context.textBetween)(leading, nodes[0]));

  // tslint:disable-next-line:max-line-length
  return new OTree([leadingNewline ? '\n' : ''], preserveSeparatingNewlines(elements, nodes, context), { separator: ', ' });
}

function mangleIdentifier(originalIdentifier: string) {
  if (startsWithUppercase(originalIdentifier)) {
    // Probably a class, leave as-is
    return originalIdentifier;
  } else {
    // Turn into snake-case
    return originalIdentifier.replace(/[^A-Z][A-Z]/g, m => m[0].substr(0, 1) + '_' + m.substr(1).toLowerCase());
  }
}

const BUILTIN_FUNCTIONS: {[key: string]: string} = {
  'console.log': 'print',
  'console.error': 'sys.stderr.write',
  'Math.random': 'random.random'
};

const TOKEN_REWRITES: {[key: string]: string} = {
  this: 'self',
  true: 'True',
  false: 'False'
};

function flat<A>(xs: A[][]): A[] {
  return Array.prototype.concat.apply([], xs);
}

function last<A>(xs: ReadonlyArray<A>): A {
  return xs[xs.length - 1];
}