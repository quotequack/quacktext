const { invoke } = window.__TAURI__.core;

const input = document.getElementById('input-id');
const generate = document.getElementById('gen');
const output = document.getElementById('output');
const dropdown = document.getElementById('type');

let inputValue;
let selectedValue;

input.addEventListener('input', () => {
  inputValue = input.value;
});

generate.addEventListener('click', async () => {
  selectedValue = parseInt(dropdown.value, 10);
  if (!inputValue) {
    output.innerHTML = `ERROR: Input cannot be empty.`;
    return;
  }

  try {
    const fnoutput = await invoke('quack', { input: inputValue, trans: selectedValue });
    output.innerHTML = `OUTPUT: ${fnoutput}`;
  } catch (error) {
    console.error("Error invoking quack:", error);
    output.innerHTML = `ERROR: ${error.message || "Unknown error"}`;
  }
});