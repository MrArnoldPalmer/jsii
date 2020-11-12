import { CodeMaker } from 'codemaker';

import { GoProperty } from '../types';
import {
  JSII_GET_FUNC,
  JSII_SET_FUNC,
  JSII_SGET_FUNC,
  JSII_SSET_FUNC,
} from './constants';
import { slugify, emitInitialization } from './util';

export class GetProperty {
  public constructor(public readonly parent: GoProperty) {}

  public emit(code: CodeMaker) {
    const resultVar = slugify('returns', [this.parent.instanceArg]);
    code.line(`var ${resultVar} ${this.parent.returnType}`);

    code.open(`${JSII_GET_FUNC}(`);
    code.line(`${this.parent.instanceArg},`);
    code.line(`"${this.parent.property.name}",`);
    code.line(`&${resultVar},`);
    code.close(`)`);

    code.line(`return ${resultVar}`);
  }
}

export class SetProperty {
  public constructor(public readonly parent: GoProperty) {}

  public emit(code: CodeMaker) {
    code.open(`${JSII_SET_FUNC}(`);
    code.line(`${this.parent.instanceArg},`);
    code.line(`"${this.parent.property.name}",`);
    code.line(`val,`);
    code.close(`)`);
  }
}

export class StaticGetProperty {
  public constructor(public readonly parent: GoProperty) {}

  public emit(code: CodeMaker) {
    emitInitialization(code);
    const resultVar = slugify('returns', []);
    code.line(`var ${resultVar} ${this.parent.returnType}`);

    code.open(`${JSII_SGET_FUNC}(`);
    code.line(`"${this.parent.parent.fqn}",`);
    code.line(`"${this.parent.property.name}",`);
    code.line(`&${resultVar},`);
    code.close(`)`);

    code.line(`return ${resultVar}`);
  }
}

export class StaticSetProperty {
  public constructor(public readonly parent: GoProperty) {}

  public emit(code: CodeMaker) {
    emitInitialization(code);

    code.open(`${JSII_SSET_FUNC}(`);
    code.line(`"${this.parent.parent.fqn}",`);
    code.line(`"${this.parent.property.name}",`);
    code.line(`val,`);
    code.close(`)`);
    code.line(`return`);
  }
}
