import { updatePreview } from './imagePreview.js'

export function readFormData() {
    const imageFile = document.getElementById("input-file").files[0];
    const socialPlatformName = document.getElementById("social-platform").value;
    const rotation = document.getElementById("rotation").value;
    const conversionFormat = document.getElementById("conversion-format").value;
    const brightness = document.getElementById("brightness").value;
    const contrast = document.getElementById("contrast").value;
    const greyscale = document.getElementById("greyscale").checked;

    updatePreview(imageFile, rotation, socialPlatformName);

    return { imageFile, socialPlatformName, rotation, conversionFormat, brightness, contrast, greyscale };
}