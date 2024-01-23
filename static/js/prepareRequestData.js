import { readFormData } from './readFormData.js';
import { encodeImageFile } from './encodeImageFile.js';

export async function prepareRequestData() {
  const { imageFile, socialPlatformName, rotation, conversionFormat, brightness, contrast, greyscale } = readFormData();
  const imageData = await encodeImageFile(imageFile);

  return {
    input_data: imageData,
    social_platform_name: socialPlatformName,
    rotation: rotation,
    format: conversionFormat,
    brightness: Number(brightness),
    contrast: Number(contrast),
    greyscale: greyscale,
  };
}