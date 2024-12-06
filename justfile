default:
    @just --list

fetch-inputs year day:
    nu fetch-inputs {{year}} {{day}}
