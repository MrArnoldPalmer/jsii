import { CodeMaker } from 'codemaker';
import { Assembly } from 'jsii-reflect';
import { EmitContext } from './emit-context';
import { ReadmeFile } from './readme-file';
import { Module, RootModule, Submodule } from './module';

export class Package {
  public readonly packageName: string;
  public readonly rootModule: RootModule;

  public constructor(public readonly assembly: Assembly) {
    this.packageName = this.assembly.name
      .replace('@', '')
      .replace(/[^a-z0-9_.]/gi, '');
    this.rootModule = new RootModule(assembly);
  }

  public emit({ code }: EmitContext): void {
    if (this.assembly.readme?.markdown) {
      new ReadmeFile(this.packageName, this.assembly.readme.markdown);
    }

    this.rootModule.emit(code);
  }

  public emitSubmodules(
    code: CodeMaker,
    parent: Module,
    parentPath = '',
  ): void {
    parent.submodules.forEach((submodule: Submodule) => {
      const nextPath = `${parentPath}/${submodule.packageName}`;
      const packageName = nextPath.replace(/^\//, '');
      const filename = `${nextPath}.go`;

      code.openFile(filename);
      code.line(`package ${packageName}`);
      code.line();
      submodule.emit(code);
      code.closeFile(filename);

      // recurse into next level
      this.emitSubmodules(code, submodule, nextPath);
    });
  }
}
