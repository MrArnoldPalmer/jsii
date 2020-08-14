import { CodeMaker } from 'codemaker';
import { InterfaceType, Method, Property } from 'jsii-reflect';
import { GoType, GoEmitter } from './go-type';
import { GoTypeRef } from './go-type-reference';
import { Module } from '../module';

export interface InterfaceField {
  name: string;
  parent: Interface;
}

export class InterfaceProperty implements InterfaceField {
  public readonly name: string;
  public readonly references?: GoTypeRef;
  public constructor(
    public parent: Interface,
    public readonly property: Property,
  ) {
    this.name = this.property.name;

    if (this.property.type) {
      this.references = new GoTypeRef(parent.parent.root, this.property.type);
    }
  }

  public emit(code: CodeMaker) {
    const name = this.property.name;
    const type =
      this?.references?.scopedName(this.parent.parent) ??
      this.property.toString();

    code.line(`get${name}() ${type}`);
    if (!this.property.protected) {
      code.line(`set${name}()`);
    }
  }
}

export class InterfaceMethod implements InterfaceField {
  public readonly name: string;
  public readonly references?: GoTypeRef;
  public constructor(public parent: Interface, public readonly method: Method) {
    this.name = this.method.name;

    if (this.method.returns.type) {
      this.references = new GoTypeRef(
        parent.parent.root,
        this.method.returns.type,
      );
    }
  }

  public emit(code: CodeMaker) {
    const returns = this.method.returns.type.void
      ? ''
      : this?.references?.scopedName(this.parent.parent) ??
        this.method.returns.toString();
    code.line(`${this.method.name}()${returns}`);
  }
}

export class Interface extends GoType implements GoEmitter {
  public readonly properties: InterfaceProperty[];
  public readonly methods: InterfaceMethod[];

  public constructor(parent: Module, public type: InterfaceType) {
    super(parent, type);

    this.properties = Object.values(this.type.getProperties()).map(
      (prop) => new InterfaceProperty(this, prop),
    );

    this.methods = Object.values(this.type.getMethods()).map(
      (method) => new InterfaceMethod(this, method),
    );
  }

  public emit(code: CodeMaker) {
    code.openBlock(`type ${this.name} interface`);

    this.properties.forEach((property) => property.emit(code));
    this.methods.forEach((method) => method.emit(code));

    code.closeBlock();
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
