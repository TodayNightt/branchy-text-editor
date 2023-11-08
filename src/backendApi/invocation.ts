import {
  ChangesRange,
  handleFileChanges,
  saveFile,
  getSourceCodeIfAny,
} from "./bindings";

//https://gist.github.com/karlhorky/3593d8cd9779cf9313f9852c59260642
export async function catchIfAny<T>(promise: Promise<T>): Promise<T | null> {
  try {
    return await promise;
  } catch (error) {
    console.log(error);
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


