#import "../utils/main.typ": to-string, hr

#let default_author = "柳下川"

#let conf(
  title: none,
  date: none,
  update: none,
  outline: true,
  series: none,
  next: none,
  prev: none,
  abstract: none,
  body,
) = {
  // metadata
  [#metadata((
    title: title,
    date: date,
    series: series,
    abstract: abstract,
  )) <front_matter>]


  html.elem("link", attrs: (rel: "stylesheet", href: "/assets/styles/highlight.min.css"))
  html.elem("script", attrs: (src: "/assets/scripts/highlight.min.js"))
  html.elem("script", attrs: (src: "/assets/scripts/rust.min.js"))
  html.elem("script", attrs: (src: "/assets/scripts/scheme.min.js"))
  html.elem("script", "hljs.highlightAll();")
  html.elem("link", attrs: (rel: "stylesheet", href: "/assets/styles/catppuccin-macchiato.css"))
  html.elem("script", attrs: (src: "/assets/scripts/htmx.min.js"))
  html.elem("link", attrs: (href: "/assets/styles/tailwind.css", rel: "stylesheet"))
  html.elem("link", attrs: (href: "/assets/iconfonts/iconfont.css", rel: "stylesheet"))
  

  // let navbar = html.elem("div", attrs: (class: "border-b-2 w-full border-slate-700 bg-ink-purple z-10 fixed top-0"),
  //   html.elem("div", attrs: (class: "flex flex-row w-fit mx-auto *:m-1 sm:w-auto sm:*:mx-2 "), [
  //     #let class = "hover:text-fuchsia-400"
  //     #html.elem("a", attrs: (class: class, href: "/"), "首页")
  //     #html.elem("a", attrs: (class: class, href: "/programming", hx-get: "/programming" hx-swap="outerHtml"), "编程")
  //     #html.elem("a", attrs: (class: class, href: "/"), "编程")
  //     #html.elem("a", attrs: (class: class, href: "/"), "编程")
  //     #html.elem("a", attrs: (class: class, href: "/"), "编程")

  //   ])
  // )
  
  let title = if title != none {
    html.elem("div", attrs: (class: "text-3xl my-4 w-full justify-center flex"), [『#title』])
  }
  let subtitle = if (date != none) or (series != none) {
    html.elem("div", attrs: (class: "font-medium w-fit mx-auto align-center flex "), {
      let separator = html.elem("span", attrs: (class: "px-2"), "│")
      if date != none { "写自:" + date + separator } + if update != none { "更新:" + update + separator } + "作者:" + default_author + if series != none {
        separator + "系列:" + html.elem("a", attrs: (class: "underline underline-offset-5", href: "/categories/" + series), series)
      }
    })
  }
  let abstract = if abstract != none {
     html.elem("div", attrs: (class: "flex flex-col items-center m-4"), [
      #html.elem("div", attrs: (class: "w-fit mx-auto text-lg font-semibold"), "摘要")
      #html.elem("div", attrs: (class: "w-fit break-keep text-center mx-[30%] *:inline"), abstract)
    ])
  }
  let footer = if (next != none) or (prev != none) {
    hr
    html.elem("div", attrs: (class: "flex flex-col m-4 *:my-2"), context {
      if prev != none {
        html.elem("a", attrs: (class: "flex items-center border-2 border-slate-500 w-full rounded-lg justify-start hover:border-white", href: prev.dest), [
          #html.elem("span", attrs: (class: "text-4xl p-2"), "")
          #html.elem("span", attrs: (class: "text-2xl p-2"), prev.body)
        ])
      }
      if next != none {
        html.elem("a", attrs: (class: "flex items-center border-2 border-slate-500 w-full rounded-lg justify-end hover:border-white", href: next.dest), [
          #html.elem("span", attrs: (class: "text-2xl p-2"), next.body)
          #html.elem("span", attrs: (class: "text-4xl p-2"), "")
        ])
      }
    })
  }
  let custom_outline = if outline { context { if query(heading).len() != 0 { html.elem("div", attrs: (class: "flex flex-col my-10 w-fit border-2 border-slate-500"), [
    #html.elem("span", attrs: (class: "text-2xl"), "目录:")
    #html.elem("div", attrs: (class: "w-fit pr-2 my-2"),
      context {
        let hs = query(selector(heading))
        let lens = hs.len()

        for i in range(lens) {
          let h = hs.at(i)
          let next = i + 1

          let is_h1 = hs.at(i).level == 1
          let is_h2 = hs.at(i).level == 2
          let is_last_h2 = (next >= lens) or (hs.at(i).level > hs.at(next).level)

          let prefix = if i == 0 {
            "┌─"
          } else if i < lens - 1 {
            if is_h1 { "├─" } else { "├───" }
          } else {
            if is_h1 { "└─" } else { "└───" }
          }

          let id = to-string(h.body).replace("/", "-")
          let anchor = html.elem("a", attrs: (class: "text-sm hover:text-sky-500 hover:font-semibold hover:underline underline-offset-5", href: "#" + id), id)
          let prefix = html.elem("span", attrs: (class: "font-bold text-sm"), prefix)
          let text = html.elem("span", attrs: (class: "pl-2"), [#prefix#anchor])
          html.elem("div", attrs: (class: "leading-5"), text)
        }
      }
  )])}} }

  set text(
    font: "Maple Mono",
  )
 
  show list: it => html.elem("ul", attrs: (class: "list-disc ml-4"), [
    #for i in it.children {
      html.elem("li", i.body)
    }
  ])
  show enum: it => html.elem("ol", attrs: (class: "list-decimal ml-7"), [
    #for i in it.children {
      html.elem("li", i.body)
    }
  ])
  
  show heading.where(level: 1): it => {
    let id = if to-string(it.supplement) != "Section" { it.supplement } else { it.body }
    let id = to-string(id).replace("/", "-")
    let link = html.elem("a", attrs: (class: "text-4xl hover:underline underline-offset-8", href: "#" + id), "" + to-string(it.body))
    [
      #hr
      #html.elem("h1", attrs: (class: "flex items-center w-fit my-6", id: id), [
        #html.elem("span", attrs: (class: "text-2xl mr-2 text-cyan-500"), "🔗") #link
      ])
    ]
  }

  show heading.where(level: 2): it => {
    let id = to-string(it.body).replace("/", "-")
    let link = html.elem("a", attrs: (class: "text-2xl hover:underline underline-offset-8", href: "#" + id), "" + to-string(it.body))
    [
      #html.elem("h2", attrs: (class: "flex items-center w-fit my-6", id: id), [
        #html.elem("span", attrs: (class: "text-2xl m-2 text-cyan-500"), "> ") #link
      ])
    ]
  }

  show table: it => html.elem("div", attrs: (class: "my-4 mx-6"), html.frame(it))
  set table(
    stroke: white,
  )
  show table: set text(
    fill: white,
    size: 13pt,
  )
  
  show math.equation: set text(
    fill: white,
    weight: "bold",
    size: 16pt,
  )
  show math.equation: it => context {
    // only wrap in frame on html export
    if target() == "html" {
      // wrap frames of inline equations in a box
      // so they don't interrupt the paragraph
      show: if it.block { it => it } else { box }
      html.elem("div", attrs: (class: "m-4 flex w-fit mx-auto"), html.frame(it))
    } else {
      it
    }
  }


  show raw.where(block: false): it => html.elem("code", attrs: (class: "font-semibold text-[#aa8bd5]"), it)
  show raw.where(block: true): it => html.elem("pre", attrs: (class: "group relative mx-6 my-4 border-2 border-slate-500 leading-[1.2em]"),[
    #html.elem("script", "
        function copyCode(button) {
          const code = button.parentElement.querySelector('code').innerText;
          navigator.clipboard.writeText(code).then(() => {
            button.textContent = '已复制!';
            setTimeout(() => button.textContent = '复制', 2000);
          });
        }
    ");
    #html.elem("code", attrs: (class: "" + " language-"+it.lang), it.text)
    #html.elem("button", attrs: (class: "absolute top-2 right-2 hidden group-hover:inline text-xl", onclick: "copyCode(this)"), "复制")
  ])

  show strike: it => box(
    html.elem("del", attrs: (class: ""), it.body)
  )
  show quote: it => html.elem("blockquote", attrs: (class: "border-l-4 border-solid border-gray-300 mx-1 my-3 px-4 opacity-60"), it.body)
  show link: it => html.elem("a", attrs: (class: "underline underline-offset-5 font-medium", href: it.dest), it.body)
  // show image: it => html.elem("img", attrs: (src: it.source, class: "mx-auto w-1/2"))
  show image: html.frame

  // navbar
  html.elem("div", attrs: (id: "content", class: "mx-4 mb-20"), [
    #title
    #subtitle
    #abstract
    #custom_outline
    #body
    #footer
  ])
}
