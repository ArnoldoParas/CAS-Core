#set page(
  paper: "a4",
  margin: (top: 1.25cm, bottom: 1.25cm),
  header: [
    #set text(font: "Times New Roman", 8pt)
    #h(1fr)
    IT-7-VIN-01-R03
  ],
  footer: context [
    #set text(font: "Times New Roman", 8pt)
    #align(left)[
      Revisión No. 7
      #linebreak()
      Vigente a partir del 01 de Agosto 2016
    ]
  ]
)
#set text(font: "Tahoma", size: 12pt)

#grid(
  columns: (2.54cm, 1fr, 2.54cm),
  image("./src/assets/img/logo_uanl.png", width: 2.80cm, height: 2.90cm),
  stack(
    spacing: 10pt,
    align(center, text(16pt)[
      *Subdirección de Vinculación*
      #v(5pt, weak: true)
      *Centro de Atención y Servicios*
    ]),
    align(center, text(13pt)[
      Teléfonos de Atención a Cliente
      #v(5pt, weak: true)
      83-29-40-20 Ext.: 5781
    ]),
  ),
  image("./src/assets/img/logo_fime.png", width: 2.54cm, height: 2.90cm),
)

#line(length: 100%)

#align(center, text(13pt)[
  *Orden De Servicio Preventivo*
])

#grid(
  columns: (60%, 40%),
  gutter: 50pt,
  stack(
    spacing: 10pt,
    grid(
      columns: (2),
      gutter: 5pt,
      [*Dependencia:*],
      repeat("_")
    ),
    grid(
      columns: (2),
      gutter: 5pt,
      [*Titular:*],
      repeat("_")
    ),
    grid(
      columns: (2),
      gutter: 5pt,
      [*Departamento:*],
      repeat("_")
    ),
    grid(
      columns: (60%, 40%),
      gutter: 5pt,
      [*Cantidad Total De Equipos:*],
      [{{TOTAL_EQUIPOS}}]
    ),
    grid(
      columns: (2),
      gutter: 5pt,
      [*Equipos Atendidos:*],
      repeat("_")
    ),
  ),
  grid(
    columns: (30%, 40%),
    gutter: 5pt,
    align(right, [
      *Fecha:*
    ]),
    repeat("_")
  )
)

*EQUIPOS*
#v(5pt)

#grid(
  {{COLUMN_CONFIG}},
  gutter: 1pt,
  {{CONTENT_COLUMNS}}
)

#v(0.5cm, weak: true)
#set text(font: "Times New Roman", 12pt)
En caso de cancelación del servicio escriba su motivo:
#box(width: 1fr, repeat("_"))
#v(0.5cm, weak: true)
#box(width: 1fr, repeat("_"))

#v(1cm, weak: true)
#grid(
  columns: (1fr, 1fr),
  gutter: 2cm,
  stack(
    spacing: 10pt,
    repeat("_"),
    align(center)[
      #text(weight: "bold")[RESPONSABLE]
    ]
  ),
  stack(
    spacing: 10pt,
    repeat("_"),
    align(center)[
      #text(weight: "bold")[TITULAR]
    ]
  ),
)