import { CodeMaker } from 'codemaker';
import { Assembly, Submodule as JsiiSubmodule } from 'jsii-reflect';
import { GoClass, Enum, Struct, Interface } from './types';

export interface ModuleTypes {
  [fqn: string]: Interface | Enum | GoClass | Struct;
}

export class Module {
  public readonly types: ModuleTypes = {};
  public readonly submodules: Submodule[];
  public readonly assembly: Assembly | JsiiSubmodule;

  public constructor(assembly: Assembly, submodule?: JsiiSubmodule) {
    this.assembly = submodule ?? assembly;
    assembly.types.forEach((type) => {
      let t: Enum | Interface | GoClass | Struct | undefined;
      if (type.isInterfaceType() && type.datatype) {
        t = new Struct(type);
      } else if (type.isInterfaceType()) {
        t = new Interface(type);
      } else if (type.isClassType()) {
        t = new GoClass(type);
      } else if (type.isEnumType()) {
        t = new Enum(type);
      }

      if (t) {
        this.types[type.fqn] = t;
      }
    });

    this.submodules = this.buildSubmodules(assembly, submodule);
  }

  public buildSubmodules(
    assembly: Assembly,
    submodule?: JsiiSubmodule,
  ): Submodule[] {
    return (submodule?.submodules ?? assembly.submodules).map(
      (sm) => new Submodule(assembly, sm),
    );
  }

  public emitTypes(code: CodeMaker) {
    Object.values(this.types).forEach((type) => {
      type.emit(code);
    });

    //     this.submodules.forEach(submodule => {
    //       code.openFile(submodule.filename);
    //       submodule.emitTypes(code);
    //       code.closeFile(submodule.filename);
    //     });
  }
}

export class Submodule extends Module {
  public constructor(assembly: Assembly, submodule: JsiiSubmodule) {
    super(assembly, submodule);
  }

  public get filename(): string {
    return 'submodulename';
  }
}
