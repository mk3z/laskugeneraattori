// https://github.com/typst/typst/issues/3269
#let price(number) = {
  let chunks = ()
  let integer = int(number)
  while integer > 1000 {
    let n = str(int(calc.rem(integer, 1000)))
    let chunk = ""
    for i in range(3 - n.len()) {
      chunk += "0"
    }
    chunk += n
    chunks.push(chunk)
    integer = int(integer / 1000);
  }

  chunks.push(str(integer))

  let s = ""
  for chunk in chunks.rev() {
    s += str(chunk)
    s += " "
  }
  s = s.slice(0, s.len() -1)

  let decimal = str(int(calc.rem(number * 100, 100)))
  s += ","

  for i in range(2 - decimal.len()){
    s = s + "0"
  }
  s += decimal
  s
}

#set page(
  background: [
    #image("tik.png")
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
*ID*: #data.id \
*Päivämäärä*: #data.creation_time \
]

== Tietokilta

*Aihe*: #data.subject \
*Perustelut*: #data.description \

=== Erittely
#let rows = data.rows.map(it => ([#it.product], [#it.quantity #it.unit],
      [#price(it.unit_price/100) €], [#price(it.quantity*it.unit_price/100) €]))
#table(columns: (55%, 15%, 15%, 15%),
  align: (left, right, right, right),
  table.header([*Tuote*], [*Määrä*],  [*Hinta per*], [*Yhteensä*]),
  ..rows.flatten(),
  ..([], [], [], [*#price(data.rows.map(r => r.unit_price*r.quantity).sum()/100) €*])
)

*IBAN-tilinumero*: #data.bank_account_number \

=== LIITTEET
#table(columns: (33%, 66%),
  table.header([*Tiedosto*], [*Kuvaus*]),
  ..data.attachments.map(a => (a.filename, a.description)).flatten()
)

#for file in data.attachments {
  if regex(".*\.(jpg|png)$") in file.filename {
    pagebreak()
    image(file.hash + "/" + file.filename)
  }
}
