import { Toast, toaster } from "@kobalte/core";
import { Component } from "solid-js";
import { BackendError } from "../../backendApi/invocation";

export const showToast = (error: unknown) => {
  if (isBackendError(error)) {
    toaster.show((props) => <Toaster toastId={props} content={error} />);
  }
};

const isBackendError = (error: unknown): error is BackendError => {
  if (typeof error !== "object" || error === null) {
    return false;
  }

  // You can add additional checks if needed

  return true;
};

const Toaster: Component<{
  toastId: Toast.ToastComponentProps;
  content: BackendError;
}> = (props) => {
  return (
    <Toast.Root toastId={props.toastId.toastId} class="toast">
      <div class="toast__content">
        <div>
          <Toast.Title class="toast__title">{props.content.kind}</Toast.Title>
          <Toast.Description class="toast__description">
            {props.content.message}
          </Toast.Description>
        </div>
        <Toast.CloseButton class="toast__close-button">X</Toast.CloseButton>
      </div>
    </Toast.Root>
  );
};

export default Toaster;
