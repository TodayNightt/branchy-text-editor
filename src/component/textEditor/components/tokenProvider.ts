import { Lang } from "../../../backendApi/bindings";
import {
  invokeGetTokensLegend,
  invokeHighlights,
} from "../../../backendApi/invocation";
import * as monaco from "monaco-editor/esm/vs/editor/editor.api";
import { store } from "../../../backendApi/stateStore";

export async function registerSemanticTokenProvider(
  language: Lang,
  id: number,
  langId: string
) {
  if (store.supportedLanguage.find((val) => val === language)) {
    const legend = await invokeGetTokensLegend(language)!;
    return monaco.languages.registerDocumentSemanticTokensProvider(langId, {
      getLegend() {
        return {
          tokenTypes: legend!._token_types,
          tokenModifiers: legend!._token_modifier,
        };
      },
      //@ts-ignore
      provideDocumentSemanticTokens: async function (
        model,
        _lastResultId,
        _token
      ) {
        // const rangedSourceCode = model.getValueInRange(range);
        const rangedSourceCode = model.getValue();
        const range = model.getFullModelRange();

        // console.log([
        //   range.startLineNumber - 1,
        //   range.startColumn - 1,
        //   range.endLineNumber - 1,
        //   range.endColumn - 1,
        // ]);

        const data = await invokeHighlights(id, rangedSourceCode, [
          range.startLineNumber - 1,
          range.startColumn - 1,
          range.endLineNumber - 1,
          range.endColumn - 1,
        ]);

        // for (let i = 3; i < data.length; i += 5) {
        //   console.log(
        //     `(${data[i - 3]},${data[i - 2]}) length: ${data[i - 1]} ${
        //       this.getLegend().tokenTypes[data[i]]
        //     }`
        //   );
        // }

        return {
          data: new Uint32Array(data),
          resultId: null,
        };
      },
      releaseDocumentSemanticTokens: function (_resultId) {},
    });
  }
}
