import {
  ChangesRange,
  commands, Lang,
    Response,
  RangePoint, SemanticLegend,
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

export async function catchResponse<T>(promise: Promise<Response<T>>): Promise<T | null> {
    try {
        const result = await promise;
        if (checkIfError(result)) {
        throw (result as { Error: string }).Error;
        }
        return (result as { Success: T }).Success;
    } catch (error) {
        showToast(error);
    }
    return null;
}

export function checkIfError<T>(result : Response<T>) :result is { Error: string } {
  return (result as  { Error : string}).Error !== undefined;
}

export const invokeHandleFileChanges = async (
  id: number,
  cr: ChangesRange | null,
  text: string
) => {
  await catchResponse(commands.handleFileChanges(id, text, cr));
};

export const invokeSaveFile = async (id: number) => {
  await catchResponse(commands.saveFile(id));
};

export const invokeGetSourceCode = async (id: number): Promise<string> => {
  const code = await catchResponse(commands.getSourceCodeIfAny(id));
  if (code) return code;
  return "";
};

export const invokeGetTokensLegend = async (
  lang: Lang
): Promise<SemanticLegend | null> => {
  const legend = await catchResponse(commands.getTokensLegend(lang));
  if (legend) return legend;
  return null;
};

export const invokeHighlights = async (
  id: number,
  ranged_source_code: string,
  range: RangePoint
): Promise<number[]> => {
  const data = await catchResponse(commands.setHighlights(id, ranged_source_code, range));
  if (data) return data;
  return [];
};
