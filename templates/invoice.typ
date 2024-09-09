#let price(number) = {
  let digits = ""
  while number > 0 {
    digits += str(calc.rem(number, 10))
    number = int(number/10)
  }

  let s = ""
  let n = 0
  for d in digits {
    if n == 2 {
      s = "," + s
    }

    if n > 2 and calc.rem(n - 2, 3) == 0 {
      s = " " + s
    }

    s = d + s
    n += 1
  }
  s
}

#set page(
  background: [
    #image("/tik.png")
  ],
  footer: [
    Laskut hyväksytään Tietokillan hallituksen kokouksissa.
    Ongelmatapauksissa ota yhteyttä rahastonhoitajaan: #link("mailto:rahastonhoitaja@tietokilta.fi").
    Tarkemmat yhteystiedot löydät killan sivuilta.
  ],
)
#set text(lang: "fi")

#let writeline(length) = {
  line(length: length, start: (0pt, 1em))
}

#move(dx: -10%, dy: -5%, box(
  width: 120%,
  inset: 1em,
  stroke: black,
)[
  #let year = datetime.today().year()
  == Rahastonhoitajan merkintöjä:
  #stack(dir: ltr)[Hyväksytty][
    #writeline(5em)
  ][.][
    #writeline(5em)
  ][.#year][
    #h(1em) TiKH:n kokouksessa
  ][
    #writeline(5em)
  ][/#year kohdistettavaksi tilille][
    #writeline(5em)
  ]
  #stack(dir: ltr)[Maksettu][
    #writeline(5em)
  ][.][
    #writeline(5em)
  ][.#year Pankkitili][
    #writeline(5em)
  ][Käteinen][
    #writeline(5em)
  ][#h(2em) TOSITE][
    #writeline(5em)
  ]
])

#columns(2)[
*Laskuttajan nimi*: #data.recipient_name \
*Katuosoite*: #data.address.street \
*Postinumero ja -toimipaikka*: #data.address.zip #data.address.city \
*Puhelin*: #link("tel:" + data.phone_number) \
*E-mail*: #link("mailto:" + data.recipient_email) \

#colbreak()
= LASKU
*Päivämäärä*: #datetime.today().display() \
]

== Tietokilta

*Aihe*: #data.subject \
*Perustelut*: #data.description \

=== Erittely
#let rows = data.rows.map(it => ([#it.product], [#it.quantity #it.unit],
      [#price(it.unit_price) €], [#price(it.quantity*it.unit_price) €]))
#table(columns: (55%, 15%, 15%, 15%),
  align: (left, right, right, right),
  table.header([*Tuote*], [*Määrä*],  [*Hinta per*], [*Yhteensä*]),
  ..rows.flatten(),
  ..([], [], [], [*#price(data.rows.map(r => r.unit_price*r.quantity).sum()) €*])
)

*IBAN-tilinumero*: #data.bank_account_number \

=== LIITTEET
#table(columns: (33%, 66%),
  table.header([*Tiedosto*], [*Kuvaus*]),
  ..data.attachments
    .zip(data.attachment_descriptions)
    .map(((a, d)) => (a.filename, d)).flatten()
)

#for file in data.attachments {
  if regex("(?i)\.(jpg|jpeg|png|gif|svg)$") in file.filename {
    pagebreak()
    image("/attachments/" + file.filename)
  }
}
