import { CodeMaker } from 'codemaker';

import { ClassMethod } from '../types';
import {
  JSII_INVOKE_FUNC,
  JSII_SINVOKE_FUNC,
} from './constants';
import { slugify, emitInitialization } from './util';

export class MethodCall {
  public constructor(public readonly parent: ClassMethod) {}

  public emit(code: CodeMaker) {
    if (this.inStatic) {
      this.emitStatic(code);
    } else {
      this.emitDynamic(code);
    }
  }

  private emitDynamic(code: CodeMaker) {
    if (this.returnType) {
      code.line(`var ${this.returnVarName} ${this.returnType}`);
    } else {
      code.line(`var ${this.returnVarName} interface{}`);
    }

    code.open(`${JSII_INVOKE_FUNC}(`);

    code.line(`${this.parent.instanceArg},`);
    code.line(`"${this.parent.method.name}",`);
    code.line(`${this.argsString},`);
    code.line(`${this.returnType ? 'true' : 'false'},`);
    code.line(`&returns,`);

    code.close(`)`);

    if (this.returnType) {
      code.line(`return ${this.returnVarName}`);
    }
  }

  private emitStatic(code: CodeMaker) {
    emitInitialization(code);
    if (this.returnType) {
      code.line(`var ${this.returnVarName} ${this.returnType}`);
    } else {
      code.line(`var ${this.returnVarName} interface{}`);
    }

    code.open(`${JSII_SINVOKE_FUNC}(`);

    code.line(`"${this.parent.parent.fqn}",`);
    code.line(`"${this.parent.method.name}",`);
    code.line(`${this.argsString},`);
    code.line(`${this.returnType ? 'true' : 'false'},`);
    code.line(`&returns,`);

    code.close(`)`);

    if (this.returnType) {
      code.line(`return ${this.returnVarName}`);
    }
  }

  private get returnVarName(): string {
    return slugify('returns', this.parent.parameters.map(p => p.name));
  }

  private get returnsVal(): boolean {
    return Boolean(this.parent.reference && !this.parent.reference.void);
  }

  protected get returnType(): string | undefined {
    if (this.returnsVal) {
      return this.parent.returnType
    }

    return
  }

  private get inStatic(): boolean {
    return this.parent.method.static;
  }

  private get argsString(): string {
    const argsList = this.parent.parameters
      .map((param) => param.name)
      .join(', ');
    return `[]interface{}{${argsList}}`;
  }
}
