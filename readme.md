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

Pour l'extension, le choix d'un array de floats à été fait afin car l'exercice se base
sur 24 tranches de 1h. L'index représente l'heure et la valeur le bénéfice dans cette
tranche. Pour faciliter la consommation de cette donnée par l'utilisateur de la librairie,
la méthode get_timestamps a été exposée et retourne un vecteur de Timestamps contenants les
champs 'hour' et 'amount'. Le tri des heures est laissé à l'utilisateur de la librairie mais
peut être fait facilement avec la méthode sort_by du vecteur. Il est intéressant de noter
que cette approche ne permet pas de connaître quel produit à été vendu ni à quelle heure/jour
précis il a été vendu.
