import { TypeReference } from 'jsii-reflect';
import { GoTypeRef } from './go-type-reference';
import { GoType } from './go-type';
import { RootModule } from '../module';

export class TypeMapper {
  public readonly type: GoType | void;

  public constructor(public root: RootModule, public value: TypeReference) {
    if (this.value.type) {
      this.type = new GoTypeRef(root, this.value).type;
    }
  }

  public emit() {
    // return this.value.toString();
    if (this.type) {
      return `${this.type.namespace}.${this.type.name}`;
    }

    return this.value.toString();
  }
}
