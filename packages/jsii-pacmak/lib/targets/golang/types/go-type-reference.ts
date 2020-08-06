import { TypeReference } from 'jsii-reflect';
import { Module, RootModule } from '../module';
import { GoType } from './go-type';

export class GoTypeRef {
  public constructor(
    public readonly root: RootModule,
    public readonly reference: TypeReference,
  ) {}

  public get type(): GoType {
    return this.root.findType(this.reference.fqn!);
  }

  public get name() {
    return this.type.name;
  }

  public get namespace() {
    return this.type.parent.packageName;
  }

  public scopedName(scope: Module) {
    if (scope.packageName === this.namespace) {
      return this.name;
    }
    return `${this.namespace}.${this.name}`;
  }
}
