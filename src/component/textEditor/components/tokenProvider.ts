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
    monaco.languages.registerDocumentRangeSemanticTokensProvider(langId, {
      getLegend() {
        return {
          tokenTypes: legend!._token_types,
          tokenModifiers: legend!._token_modifier,
        };
      },
      //@ts-ignore
      provideDocumentRangeSemanticTokens: async function (
        model,
        range,
        _token
      ) {
        const rangedSourceCode = model.getValueInRange(range);

        const data = await invokeHighlights(id, rangedSourceCode);

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
    });
  }
}
