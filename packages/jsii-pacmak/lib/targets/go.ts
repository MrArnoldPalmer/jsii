import * as path from 'path';
import { CodeMaker } from 'codemaker';
import { Documentation } from './go/documentation';
import { Assembly } from 'jsii-reflect';
import { Rosetta } from 'jsii-rosetta';
import { RootPackage } from './go/package';
import { IGenerator } from '../generator';
import { Target, TargetOptions } from '../target';
import { getByteSlice, goPackageName } from './go/util';
import {
  JSII_EMBEDDED_MODULE_NAME,
  JSII_EMBEDDED_TARBALL_EXPORT,
} from './go/runtime';

export class Golang extends Target {
  public readonly generator: IGenerator;

  public constructor(options: TargetOptions) {
    super(options);

    this.generator = new GoGenerator(options.rosetta);
  }

  /**
   * Generates a publishable artifact in `outDir`.
   *
   * @param sourceDir the directory where the generated source is located.
   * @param outDir    the directory where the publishable artifact should be placed.
   */
  public async build(sourceDir: string, outDir: string): Promise<void> {
    return this.copyFiles(sourceDir, outDir);
  }
}

class GoGenerator implements IGenerator {
  private assembly!: Assembly;
  private readonly code = new CodeMaker();
  private readonly documenter: Documentation;

  public constructor(private readonly rosetta: Rosetta) {
    this.documenter = new Documentation(this.code, this.rosetta);
  }

  public async load(_: string, assembly: Assembly): Promise<void> {
    this.assembly = assembly;
    return Promise.resolve();
  }

  public async upToDate(_outDir: string) {
    return Promise.resolve(false);
  }

  public generate(): void {
    new RootPackage(this.assembly).emit({
      code: this.code,
      documenter: this.documenter,
    });
  }

  public async save(outDir: string, tarball: string): Promise<any> {
    const output = path.join(outDir, goPackageName(this.assembly.name));
    this.embedTarball(tarball);
    await this.code.save(output);
  }

  private embedTarball(tarball: string) {
    const filename = `${JSII_EMBEDDED_MODULE_NAME}/${JSII_EMBEDDED_MODULE_NAME}.go`;
    const tarballSlice = getByteSlice(tarball);
    this.code.openFile(filename);
    this.code.line(`package ${JSII_EMBEDDED_MODULE_NAME}`);
    this.code.line();

    this.code.line(
      `var ${JSII_EMBEDDED_TARBALL_EXPORT} = []byte {${tarballSlice.join(
        ', ',
      )}}`,
    );

    this.code.closeFile(filename);
  }
}
