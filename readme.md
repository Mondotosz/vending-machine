# Code d'entretien ES 2023

Ce répertoire contient l'implémentation de l'exercice d'entretien pour la formation
d'ES 2023 à Sainte-Croix.

[Consigne](https://github.com/CPNV-ES/2023_Code_Entretien/blob/main/LISEZMOI.pdf)

## Commentaires

Les valeurs monétaires sont représentées par des floats par facilité mais n'est pas
optimal.  A cause des erreurs d'arrondi, les tests ont du être adaptés afin de prendre
une marge d'erreur de 1e-6 (0.000001).

Les articles ne sont pas exposés aux utilisateurs de la librairie. Ils sont définis lors
de l'instanciation de 'VendingMachine' et ne peuvent pas être retoqués ou ajoutés.
