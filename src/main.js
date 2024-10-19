const { invoke } = window.__TAURI__.core;

const input = document.getElementById('input-id');
const generate = document.getElementById('gen');
const output = document.getElementById('output');

let inputValue;
let selectedValue;

input.addEventListener('input', () => {
  inputValue = input.value;
});



generate.addEventListener('click', async () => {
  try {
    const fnoutput = await invoke('quack', { input: inputValue});
    output.innerHTML = `OUTPUT: ${fnoutput}`;
  } catch (error) {
    console.error("Error invoking turn_quack:", error);
    output.innerHTML = `ERROR: ${error.message}`;
  }
});