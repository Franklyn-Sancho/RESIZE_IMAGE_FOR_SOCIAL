import { prepareRequestData } from './prepareRequestData.js';

async function sendRotateResizeRequest(data) {
  return fetch("/get-image", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify(data),
  });
}

async function downloadImage(data) {
  const response = await sendRotateResizeRequest(data);
  const blob = await response.blob();

  const href = URL.createObjectURL(blob);
  const anchor = document.createElement("a");
  anchor.href = href;

  const extension = {
    jpeg: ".jpeg",
    png: ".png",
  }[data.format] || ".jpg";
  const filename = `rotated_resized_image${extension}`;

  anchor.download = filename;
  document.body.appendChild(anchor);
  anchor.click();
}

document
  .getElementById("image-form")
  .addEventListener("submit", async (event) => {
    event.preventDefault();

    try {
      let data = await prepareRequestData();
      await downloadImage(data);
      document.getElementById("result").textContent = "successfully downloaded image";
    } catch (error) {
      console.error(error);
      document.getElementById("result").textContent = error.message;
    }
  });