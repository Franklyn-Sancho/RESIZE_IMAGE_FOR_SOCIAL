let brightnessButton = document.querySelector("#brightness-button");
let brightnessContainer = document.querySelector("#brightness-container");
let contrastButton = document.querySelector("#contrast-button");
let contrastContainer = document.querySelector("#contrast-container");
let greyscaleButton = document.querySelector("#greyscale-button");
let greyscaleContainer = document.querySelector("#greyscale-container");

brightnessButton.addEventListener("click", () => {
    toggleContainer(brightnessContainer);
});

contrastButton.addEventListener("click", () => {
    toggleContainer(contrastContainer);
});

greyscaleButton.addEventListener("click", () => {
    toggleContainer(greyscaleContainer);
});

document.addEventListener("DOMContentLoaded", () => {
    const brightnessInput = document.querySelector("#brightness");
    const brightnessOutput = document.querySelector("output[for=brightness]");
    updateOutput(brightnessInput, brightnessOutput);

    const contrastInput = document.querySelector("#contrast");
    const contrastOutput = document.querySelector("output[for=contrast]");
    updateOutput(contrastInput, contrastOutput);
});

function toggleContainer(container) {
    if (container.style.display === "none") {
        container.style.display = "block";
    } else {
        container.style.display = "none";
    }
}

function updateOutput(input, output) {
    input.addEventListener("input", () => {
        output.textContent = input.value;
    });
}