#let to-string(content) = {
  if type(content) == none {
    return ""
  }
  if type(content) == bool {
    return bool-to-str(content)
  }
  if type(content) == str {
    return content
  }
  if content.has("text") {
    if type(content.text) == str {
      content.text
    } else {
      to-string(content.text)
    }
  } else if content.has("children") {
    content.children.map(to-string).join("")
  } else if content.has("body") {
    to-string(content.body)
  } else if content == [ ] {
    ""
  } else {
    "not-supported"
  }
}

#let hr = html.elem("hr", attrs: (class: "border-2 border-dashed my-10"))

#let html-text(
  font: none,
  style: "normal",
  weight: "regular",
  size: 100%,
  fill: none,
  tracking: 0pt,
  spacing: 0pt + 100%,
  content,
) = {
  let styles = ()
  if font != none {
    styles.push("font: " + font + ";")
  }
  if style != "normal" {
    styles.push("font-style: " + to-string([#style]) + ";")
  }
  if weight != "regular" {
    styles.push("font-weight: " + to-string([#weight]) + ";")
  }
  if size != 100% {
    styles.push("font-size: " + to-string([#size]) + ";")
  }
  if fill != none {
    styles.push("color: " + fill.to-hex() + ";")
  }
  if tracking != 0pt {
    styles.push("letter-spacing: " + to-string([#tracking]) + ";")
  }
  if spacing != 100% + 0pt {
    styles.push("word-spacing: " + to-string([#spacing]) + ";")
  }

  html.elem("span", attrs: (style: styles.join(" ")))[#content]
}


#let todo(completed, body) = html.elem("div", attrs: (class: "flex felx-row items-center my-1"), [
  #html.elem("input", attrs: (
    type: "checkbox",
    checked: "completed",
    class: {
      "w-5! h-5! text-4xl! mx-3 align-middle shrink-0" + " appearance-none outline-1 outline-solid outline-sky-300" + if completed {
        " after:inline-block after:content-['L'] after:text-cyan-200" + " after:-translate-x-[0.4px] after:-translate-y-[11px] after:rotate-40 after:-scale-x-65 after:scale-y-70"
      }
    },
  ))
  #html.elem("div", attrs: (class: ""),
    body
  )
])
