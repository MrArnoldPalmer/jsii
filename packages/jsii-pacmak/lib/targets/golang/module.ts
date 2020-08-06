import { CodeMaker } from 'codemaker';
import { Assembly, Type, Submodule as JsiiSubmodule } from 'jsii-reflect';
import { join } from 'path';
import { GoType, GoClass, Enum, Interface } from './types';

type ModuleType = Interface | Enum | GoClass;
type ModuleTypes = ModuleType[];

export interface Module {
  root: RootModule;
  packageName: string;
  file: string;
  submodules: Submodule[];
  types: ModuleTypes;
}

function buildSubmodules(
  root: RootModule,
  parent: Module,
  submodules: readonly JsiiSubmodule[],
): Submodule[] {
  return submodules.map((sm) => new Submodule(root, parent, sm));
}

function unknownType(type: Type): never {
  throw new Error(`Type: ${type.name} is not an interface, enum, or class`);
}

function buildModuleTypes(parent: Module, types: readonly Type[]): ModuleTypes {
  return types.map(
    (type: Type): ModuleType => {
      if (type.isInterfaceType()) {
        return new Interface(parent, type);
      } else if (type.isClassType()) {
        return new GoClass(parent, type);
      } else if (type.isEnumType()) {
        return new Enum(parent, type);
      }
      return unknownType(type);
    },
  );
}

type TypeMap = { [fqn: string]: ModuleType };

function buildModuleTypeMap(module: Module): TypeMap {
  const current = module.types.reduce((accum, type) => {
    const fqn = type.type.fqn;
    return { ...accum, [fqn]: type };
  }, {});
  const next = module.submodules.reduce((accum, mod) => {
    return { ...accum, ...buildModuleTypeMap(mod) };
  }, {});

  return { ...current, ...next };
}

export abstract class ModuleFile {
  public constructor(public readonly file: string) {}
  public open(code: CodeMaker): void {
    code.open(this.file);
  }

  public close(code: CodeMaker): void {
    code.close(this.file);
  }
}

export class RootModule extends ModuleFile implements Module {
  private readonly assembly: Assembly;
  public readonly packageName: string;
  public readonly file: string;
  public readonly root: RootModule;

  public constructor(assembly: Assembly) {
    const packageName = assembly.name
      .replace('@', '')
      .replace(/[^a-z0-9_.]/gi, '');
    const file = `${join(...packageName.split('.'))}.go`;
    super(file);

    this.assembly = assembly;
    this.root = this;
    this.packageName = packageName;
    this.file = file;
  }

  public get typeMap() {
    return buildModuleTypeMap(this);
  }

  public get types(): ModuleTypes {
    return buildModuleTypes(this, Object.values(this.assembly.types));
  }

  public get submodules(): Submodule[] {
    return buildSubmodules(this, this, this.assembly.submodules);
  }

  public emit(code: CodeMaker): void {
    this.open(code);
    code.line(`package ${this.packageName}`);
    code.line();
    this.emitTypes(code);
    this.close(code);
  }

  public emitTypes(code: CodeMaker) {
    Object.values(this.types).forEach((type) => {
      type.emit(code);
    });
  }

  public findType(fqn: string): GoType {
    return this.typeMap[fqn];
  }
}

export class Submodule extends ModuleFile implements Module {
  private readonly assembly: JsiiSubmodule;
  public readonly packageName: string;
  public readonly file: string;
  public readonly parent: Module;
  public readonly root: RootModule;

  public constructor(
    root: RootModule,
    parent: Module,
    submodule: JsiiSubmodule,
  ) {
    const packageName = submodule.name
      .replace('@', '')
      .replace(/[^a-z0-9_.]/gi, '');
    const file = `${join(...packageName.split('.'))}.go`;
    super(file);

    this.assembly = submodule;
    this.root = root;
    this.parent = parent;
    this.packageName = packageName;
    this.file = file;
  }

  public get types(): ModuleTypes {
    return buildModuleTypes(this, this.assembly.types);
  }

  public get submodules(): Submodule[] {
    return buildSubmodules(this.root, this, this.assembly.submodules);
  }

  public emit(code: CodeMaker): void {
    this.open(code);
    code.line(`package ${this.packageName}`);
    code.line();
    this.emitTypes(code);
    this.close(code);
  }

  private emitTypes(code: CodeMaker): void {
    this.types.forEach((type) => {
      type.emit(code);
    });
  }
}

// export class Module {
//   public readonly types: ModuleTypes = {};
//   public readonly submodules: Submodule[];
//   public readonly assembly: Assembly | JsiiSubmodule;

//   public constructor(assembly: Assembly, submodule?: JsiiSubmodule) {
//     this.assembly = submodule ?? assembly;
//     const types = submodule ? submodule.types : Object.values(assembly.types);
//     types.forEach((type) => {
//       let t: Enum | Interface | GoClass | undefined;
//       if (type.isInterfaceType()) {
//         t = new Interface(type);
//       } else if (type.isClassType()) {
//         t = new GoClass(type);
//       } else if (type.isEnumType()) {
//         t = new Enum(type);
//       }

//       if (t) {
//         this.types[type.fqn] = t;
//       }
//     });

//     this.submodules = this.buildSubmodules(assembly, submodule);
//   }

//   public buildSubmodules(
//     assembly: Assembly,
//     submodule?: JsiiSubmodule,
//   ): Submodule[] {
//     return (submodule?.submodules ?? assembly.submodules).map(
//       (sm) => new Submodule(assembly, sm),
//     );
//   }

//   public findSubmodule(name: string): Submodule | void {
//     return this.submodules.find((sm: Submodule) => sm.name === name);
//   }

//   public emit(code: CodeMaker) {
//     Object.values(this.types).forEach((type) => {
//       type.emit({ code });
//     });
//   }
// }
