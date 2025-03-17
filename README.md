# Tabëla

[<img src="https://img.shields.io/badge/Source_code-GitHub-BBAACC" alt="GitHub">](https://github.com/xanathar/tabela)

Tabëla is a very simple GNOME app that formats tables coming from spreadsheets or
CSV/TSV files into HTML or Markdown.

Input can be any text separated by TAB, commas or semicolon and it can
output the table formatted for Markdown (and ASCII art) or plain HTML.

It is typically used as a converter by pasting data coming from a spreadsheet like
LibreOffice Calc or Gnumeric, but can also be used as a simple editor.

## Usage

![Example image of Tabela's UI](https://raw.githubusercontent.com/xanathar/tabela/refs/heads/main/doc_assets/albums_md.png)

Just copy the cells you want from your spreadsheet of choice and paste them into the top area.
Markdown or HTML will appear in the bottom. Select and Ctrl+C to copy the result.

## FAQ

> Why did you write this program?

I wish I didn't have to format tables so many times, to paste it on Slack, on Jira tickets, on wikis, or
wherever.

> What does Tabëla mean?

"Tabëla" means "table" (as in 'spreadsheet table', not 'the surface where you place dishes' table) in
[Piedmontese](https://en.wikipedia.org/wiki/Piedmontese_language). I'm from the Piedmont region of Italy,
my grandma spoke mainly Piedmontese, as do my in-laws when they talk to each other and many other relatives.
It's a dying language/dialect and is mostly used out of tech. I love using it in a tech context as a homage.
Plus, it's a little funny thing to do.

> Is the quote parsing comformant to the CSV standards?

Maybe? It should be comformant enough to handle most spreadsheets, in either CSV or TSV format, with the option
of keeping or removing the delimiting quotes.

> Is the application localized?

English and Italian. Translations are welcome, though!

> Your icon sucks

I suck at art. My best drawing pales compared to a stick figure drawn by a 3 years old. Feel free to contribute
an icon, help is welcome.

> The icon does not respect GNOME design guidelines

See above.

> How big of a file I can paste into the tool?

Pasting a [9.1MB CSV file](https://www.stats.govt.nz/assets/Uploads/Effects-of-COVID-19-on-trade/Effects-of-COVID-19-on-trade-At-15-December-2021-provisional/Download-data/effects-of-covid-19-on-trade-at-15-december-2021-provisional.csv) from the [government of New Zealand](https://www.stats.govt.nz/large-datasets/csv-files-for-download/) and the application remains snappy and responsive. That's plenty of size for the purpose of this application, and probably nobody wants to look at such a large table in Markdown anyway. Pasting a 160MB file from the same governative site makes Tabëla appear stuck only for a few seconds, but the formatting still takes place.

![Example with a lot of data](https://raw.githubusercontent.com/xanathar/tabela/refs/heads/main/doc_assets/large_file_md.png)

> Can I see an example of what the html output would be?

```html
<table>
	<tr>
		<th>Artist</th>
		<th>Album</th>
		<th>Released</th>
		<th>Length</th>
		<th>Recording Sales (mil)</th>
		<th>Claimed sales (mil)</th>
		<th>Released</th>
		<th>Soundtrack</th>
	</tr>
	<tr>
		<td>Michael Jackson</td>
		<td>Thriller</td>
		<td>1982</td>
		<td>0:42:19</td>
		<td>46</td>
		<td>65</td>
		<td>30-Nov-82</td>
	</tr>
    ...
</table>

```

or, in a screenshot:

![Example with html output](https://raw.githubusercontent.com/xanathar/tabela/refs/heads/main/doc_assets/albums_html.png)
