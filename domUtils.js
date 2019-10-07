export function appendStringToBody(value) {
  const line = document.createElement('h2')
  const text = document.createTextNode(value)
  line.appendChild(text)
  document.body.appendChild(line)
}

export function appendNumberToBody(number) {
  const line = document.createElement('p')
  const text = document.createTextNode(number)
  line.appendChild(text)
  document.body.appendChild(line)
}
