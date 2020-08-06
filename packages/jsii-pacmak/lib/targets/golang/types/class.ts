import { Method, Property, ClassType } from 'jsii-reflect';
import { CodeMaker } from 'codemaker';
import { GoTypeRef } from './go-type-reference';
import { GoType, GoEmitter } from './go-type';
import { Module } from '../module';

export class ClassField {
  public constructor(public readonly parent: GoClass) {}
}

export class ClassProperty extends ClassField {
  public readonly references?: GoTypeRef;
  public constructor(parent: GoClass, public readonly type: Property) {
    super(parent);

    if (type.type) {
      this.references = new GoTypeRef(parent.parent.root, type.type);
    }
  }
}

export class ClassMethod extends ClassField {
  public readonly references?: GoTypeRef;
  public constructor(parent: GoClass, public readonly method: Method) {
    super(parent);
    if (method.returns.type) {
      this.references = new GoTypeRef(parent.parent.root, method.returns.type);
    }
  }
}
/*
 * Class wraps a Typescript class as a Go custom struct type  TODO rename?
 */
export class GoClass extends GoType implements GoEmitter {
  public readonly properties: ClassProperty[];
  public readonly methods: ClassMethod[];
  public readonly dependencies: Module[];

  public constructor(parent: Module, public type: ClassType) {
    super(parent, type);

    this.properties = Object.values(this.type.getProperties()).map(
      (prop) => new ClassProperty(this, prop),
    );

    this.methods = Object.values(this.type.getMethods()).map(
      (method) => new ClassMethod(this, method),
    );

    this.dependencies = [
      ...this.properties.reduce((accum: Module[], property) => {
        return property.references?.type.parent
          ? [...accum, property.references?.type.parent]
          : accum;
      }, []),
      ...this.methods.reduce((accum: Module[], method) => {
        return method.references?.type.parent
          ? [...accum, method.references?.type.parent]
          : accum;
      }, []),
    ];
  }

  public emit(code: CodeMaker): void {
    code.openBlock(`type ${this.name} struct`);

    code.closeBlock();
    code.line();
  }

  // public emit({ code }: GoEmitContext): void {
  //   code.openBlock(`type ${this.localName()} struct`);

  //   Object.values(this.type.getProperties()).forEach((property) =>
  //     this.emitClassProperty(code, property),
  //   );

  //   code.closeBlock();
  //   code.line();

  //   Object.values(this.type.getMethods()).forEach((method) =>
  //     this.emitClassMethod(code, method),
  //   );
  // }

  // private emitClassProperty(code: CodeMaker, property: Property) {
  //   const type = new TypeMapper(property.type).emit();

  //   code.line(`${property.name} ${type}`);
  // }

  // private emitClassMethod(code: CodeMaker, method: Method) {
  //   const returns = method.returns.type.void
  //     ? ''
  //     : ` ${new TypeMapper(method.returns.type).emit()}`;
  //   const instanceArg = this.localName().substring(0, 1);

  //   // TODO: Method Arguments
  //   // NOTE: May need to capitalize method name
  //   code.openBlock(
  //     `func (${instanceArg} *${this.localName()}) ${method.name}()${returns}`,
  //   );
  //   code.line(`// jsiiruntime.methodcall(${instanceArg})`);
  //   code.closeBlock();
  //   code.line();
  // }
}
