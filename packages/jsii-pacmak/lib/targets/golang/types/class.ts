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
  public constructor(parent: GoClass, public readonly property: Property) {
    super(parent);

    if (property.type) {
      this.references = new GoTypeRef(parent.parent.root, property.type);
    }
  }

  public emit(code: CodeMaker) {
    const name = this.property.name;
    const type =
      this.references?.scopedName(this.parent.parent) ??
      this.property.toString();

    code.line(`${name} ${type}`);
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

  public emit(code: CodeMaker) {
    const name = this.method.name;
    const type =
      this.references?.scopedName(this.parent.parent) ?? this.method.toString();

    const instanceArg = this.parent.name.substring(0, 1);

    // TODO: Method Arguments
    // NOTE: May need to capitalize method name
    code.openBlock(
      `func (${instanceArg} *${this.parent.name}) ${name}()${type}`,
    );
    code.line(`// jsiiruntime.methodcall(${instanceArg})`);
    code.closeBlock();
    code.line();
  }
}

/*
 * Class wraps a Typescript class as a Go custom struct type  TODO rename?
 */
export class GoClass extends GoType implements GoEmitter {
  public readonly properties: ClassProperty[];
  public readonly methods: ClassMethod[];

  public constructor(parent: Module, public type: ClassType) {
    super(parent, type);

    this.properties = Object.values(this.type.getProperties()).map(
      (prop) => new ClassProperty(this, prop),
    );

    this.methods = Object.values(this.type.getMethods()).map(
      (method) => new ClassMethod(this, method),
    );
  }

  public emit(code: CodeMaker): void {
    code.openBlock(`type ${this.name} struct`);

    this.properties.forEach((property) => property.emit(code));

    code.closeBlock();
    code.line();

    this.methods.forEach((method) => method.emit(code));

    code.line();
  }

  public get dependencies(): Module[] {
    return [
      ...this.properties.reduce((accum: Module[], property) => {
        return property.references?.type?.parent
          ? [...accum, property.references?.type.parent]
          : accum;
      }, []),
      ...this.methods.reduce((accum: Module[], method) => {
        return method.references?.type?.parent
          ? [...accum, method.references?.type.parent]
          : accum;
      }, []),
    ];
  }
}
