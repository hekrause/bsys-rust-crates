# 1. Homework hw7 task1

<!-- TOC -->

- [1. Homework hw7 task1](#1-homework-hw7-task1)
    - [1.1. Ziel](#11-ziel)
    - [1.2. Protokollbeschreibung](#12-protokollbeschreibung)
        - [1.2.1. PUBLISH](#121-publish)
        - [1.2.2. RETRIEVE](#122-retrieve)
    - [1.3. Aufgabe](#13-aufgabe)
        - [1.3.1. Crate](#131-crate)
        - [1.3.2. Testabdeckung](#132-testabdeckung)
        - [1.3.3. Fehlerbehandlung und Rückgabetyp](#133-fehlerbehandlung-und-rückgabetyp)
    - [1.4. Hilfen](#14-hilfen)
    - [1.5. Externe Crates](#15-externe-crates)
    - [1.6. Module und Tests](#16-module-und-tests)
    - [1.7. Dokumentation](#17-dokumentation)
    - [1.8. Optional (keine Extrapunkte, JustForFun!)](#18-optional-keine-extrapunkte-justforfun)

<!-- /TOC -->

## 1.1. Ziel

Ziel dieser Aufgabe ist es, einen Parser für ein simples Protokoll zu implementieren, das in den Folgeaufgaben verwendet wird.

Das Protokoll beschreibt das Veröffentlichen und Abrufen von Daten aus einem Netzwerkdienst.

## 1.2. Protokollbeschreibung

Das Protokoll kennt 2 Kommandos: Veröffentlichung und Abrufen. Weitere Kommandos sind nicht zugelassen. Das Protokoll arbeitet zeilenweise, d.h. ein newline ('\n') beendet das Kommando.

Die Kommandos folgen diesem Format:

```
<AKTION> [DATEN]\n
```

Werden keine Daten erwartet, folgt das newline-Zeichen direkt auf die Aktion.

Der Zeilenumbruch ist _nicht_ optional.

### 1.2.1. PUBLISH

Das PUBLISH-Kommando veröffentlicht eine Nachricht. Die Daten dürfen selbst keine newline enthalten. Es ist erlaubt, eine _leere_ Nachricht zu veröffentlichen.

Beispiele:

```
PUBLISH Hallo, hier ist eine kleine Nachricht!\n
PUBLISH \n
```

### 1.2.2. RETRIEVE

Das RETRIEVE-Kommando entnimmt eine Nachricht. Es erhält keine Daten.

Beispiel:

```
RETRIEVE\n
```

## 1.3. Aufgabe

Implementieren Sie einen Parser für dieses Protokoll. Der Parser sollte so simpel wie möglich gehalten sein.

Es ist insbesondere erlaubt, die Eingabe als einzelne Zeile zu erwarten. Die Trennung der Eingabe in einzelne Zeilen ist dem aufrufenden Code überlassen.

Enthält der Input mehrere newlines, darf der Rest ohne Fehler verworfen werden.

Folgendes öffentliche Interface ist vorgegeben:

```rust
pub fn parse(message: &str) -> ... {
   //...
}
```

### 1.3.1. Crate

Implementieren Sie den Parser als eigene [Crate][], die Sie dann in den weiteren Aufgaben einbinden können, statt den Code zu kopieren. Sie können diese Crate später entweder auf Github veröffentlichen oder über Pfade einbinden.

### 1.3.2. Testabdeckung

Achten Sie auf eine ausreichende Testabdeckung, um möglichst alle illegalen Fälle abzudecken. Achten Sie vor allem darauf, dass keine Daten verloren gehen!

### 1.3.3. Fehlerbehandlung und Rückgabetyp

Finden Sie einen geeigneten Rückgabetyp, der das gefundene Kommando, als auch eventuelle Fehler kommuniziert. Kombinieren Sie dies mit einer geeigneten Darstellung des Kommandos.

Allokieren Sie die gefunden Daten als `String`.

## 1.4. Hilfen

Die Dokumentation von [`str`][str] enthält viele wichtige Hinweise.

## 1.5. Externe Crates

Benutzen Sie keine externen Crates.

## 1.6. Module und Tests

Ob und wie Sie den Code in weitere Module aufteilen wollen ist Ihnen überlassen. Schreiben Sie jedoch Ihre Tests in der Datei `tests/task1.rs` die von 'cargo test' aufgerufen wird, siehe auch [Testing][]. Einfache Tests können auch direkt in die Dokumentation 'codiert' werden, siehe [Documentation Tests][].

## 1.7. Dokumentation

Bei dieser Aufgabe ist Ihre Dokumentation wichtig, um Ihren Programmablauf nachvollziehen zu können. Bitte dokumentieren Sie Ihre Funktionen entsprechend umfangreicher und kommentieren Sie spezielle Kniffe im Code, die Sie verwendet haben.

## 1.8. Optional (keine Extrapunkte, JustForFun!)

Wenn Sie möchten, können Sie einmal versuchen, die Daten nicht als `String` zu allokieren. Stattdessen geben Sie `&str` zurück. Sie sollten dazu aber ein getrenntes Projekt oder einen eigenen Branch verwenden, da diese Strategie für die folgenden Aufgaben wenig Sinn macht.


[str]: https://doc.rust-lang.org/std/primitive.str.html
[Crate]: https://doc.rust-lang.org/book/crates-and-modules.html
[Testing]: https://doc.rust-lang.org/book/testing.html
[Documentation Tests]: https://doc.rust-lang.org/book/testing.html#documentation-tests
