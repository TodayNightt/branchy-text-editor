import {
  ChangesRange,
  handleFileChanges,
  saveFile,
  getSourceCodeIfAny,
  Lang,
  setHighlights,
  SemanticLegend,
  getTokensLegend,
  RangePoint,
} from "./bindings";
import { showToast } from "../component/notification_toast/toast";

export interface BackendError {
  kind: string;
  message: string;
}

//https://gist.github.com/karlhorky/3593d8cd9779cf9313f9852c59260642
export async function catchIfAny<T>(promise: Promise<T>): Promise<T | null> {
  try {
    return await promise;
  } catch (error) {
    showToast(error);
  }
  return null;
}

export const invokeHandleFileChanges = async (
  id: number,
  cr: ChangesRange | null,
  text: string
) => {
  await catchIfAny(handleFileChanges(id, text, cr));
};

export const invokeSaveFile = async (id: number) => {
  await catchIfAny(saveFile(id));
};

export const invokeGetSourceCode = async (id: number): Promise<string> => {
  const code = await catchIfAny(getSourceCodeIfAny(id));
  if (code) return code;
  return "";
};

export const invokeGetTokensLegend = async (
  lang: Lang
): Promise<SemanticLegend | null> => {
  const legend = await catchIfAny(getTokensLegend(lang));
  if (legend) return legend;
  return null;
};

export const invokeHighlights = async (
  id: number,
  ranged_source_code: string,
  range: RangePoint
): Promise<number[]> => {
  const data = await catchIfAny(setHighlights(id, ranged_source_code, range));
  if (data) return data;
  return [];
};
