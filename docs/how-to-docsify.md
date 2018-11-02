# How to docsify

## Beschreibung

Docsify ist eine JavaScript-Anwendung, die es erlaubt Markdown-Dokumente als
Webseite darzustellen. Mit den docsify-Tools kann man sehr einfach einen lokalen
Webserver erstellen, der dann die Dokumente als Webseite ausliefert.

[Docsify Webseite](https://docsify.js.org/#/)

## Markdown-Dokumente erzeugen

Die Markdown-Dokumente werden einfach im docs-Ordner des GitHub-Repositories gelagert.
Hier dürfen auch ruhig Unterverzeichnisse erstellt werden.

## Dokument zur Sidebar hinzufügen

Das Hinzufügen eines Dokuments in die Sidebar wird durch die Datei _sidebar.md realisiert.

Ein Eintrag folgender Form:

\* \[NAME\](PATH)

mit

NAME - Der Name des Eintrags in der Sidebar  
PATH - Der Pfad zum Markdown-Dokument mit dem docs-Ordner als Wurzerverzeichis

fügt einen Eintrag in die Sidebar ein.

### Beispiel

Die Datei document.md liegt im docs-Ordner. Der Name in der Sidebar soll Document heißen.

Ein Eintrag in die _sidebar.md Datei würde so heißen:

\* \[Document\](/document.md)