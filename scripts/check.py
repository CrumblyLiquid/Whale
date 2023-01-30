# Kontrola slovíček vůči např. nějakému slovníku
# Umožnění anotování vracení sama sebe (resp. typu třídy): class Monke: def fun() -> Monke: ...
from __future__ import annotations
import sys # Argumenty

# Načítání souborů slovíček
from convert import Package, get_extension

# Zpomalení checkování pro check_website() - abychom je nespamovali :D
# from time import sleep

# Scraping
import requests
from bs4 import BeautifulSoup

# Očištění slov pomocí regexu
import re

# Lokální kontrola slov
import enchant
# Používám Aspell (ArchLinux packages: aspell, aspell-de) pro kontolu
# POZOR! Nutno měnit pro různé jazyky (echang.list_dicts() vypíše všechny nainstalované slovníky)
DICT = enchant.Dict("de_DE")

def check_local(word: str) -> bool:
    return DICT.check(word)

def check_website(word: str) -> bool:
    # Parsování stránky
    site = requests.get(f"https://www.verbformen.com/?w={word}")
    soup = BeautifulSoup(site.content, "html.parser")
    res = soup.find("div", class_="rAbschnitt")

    if(res is not None):
        return True
    else:
        return False

def check_word(orig_word: str) -> bool:
    # Očistit slovo
    words = orig_word.split(" ")

    # Pokusíme se extrahovat jen samotné slovo
    if(len(words) == 1): # Samotné slovo (nejspíš sloveso nebo příd. jm)
        word = words[0]
    elif(len(words) == 2): # Člen + slovo
        word = words[1]
    elif(len(words) > 2): # Ostatní
        if(words[0] == "sich"): # Slovesa se sich
            word = words[1]
        else:
            word = words[0] # Slovesa bez sich a ostatní slova

    # Nahradit možné přídatné znaky (např / pro odlučovací předponu)
    word = re.sub(r"/|\.|\(|\)", '', word)

    print(f"Checking: {word} ({orig_word})")

    # return check_website(word)
    return check_local(word)

# Podpora CLI
if __name__ == "__main__":
    args = sys.argv[1:]
    if(len(args) > 0):
        from_file = args[0]

        # Create Package object by loading one of the supported filetypes
        pkg = Package
        from_type = Package.fn_from_ext(get_extension(from_file))
        match(from_type):
            case "text":
                pkg = Package.from_text(from_file)
            case "table":
                pkg = Package.from_table(from_file)
            case "document":
                pkg = Package.from_document(from_file)
            case "json":
                pkg = Package.from_json(from_file)

        # Počítá chyby
        problems = 0

        # Zkontrolovat všechna slova
        for multiword in pkg.words:
            word = multiword[0]
            valid = check_word(word)
            if(valid == False):
                problems += 1
                print(f"===== PROBLEM: {word} might not be valid! =====")
            # sleep(10)

        if(problems == 0):
            print("No problems found :)")
        else:
            print(f"WARNING: {problems} problems found!")
    else:
        print("ERROR: No input file provided!")