import re
import sys
from collections import Counter

def trouver_opcodes_manquants_et_doublons(chemin_fichier):
    # Les 11 opcodes invalides/non mappés de la Game Boy (Sharp SM83)
    OPCODES_INVALIDES_GB = {
        0xD3, 0xDB, 0xDD, 
        0xE3, 0xE4, 0xEB, 0xEC, 0xED, 
        0xF4, 0xFC, 0xFD, 0xCB
    }

    # Génère l'ensemble de tous les opcodes théoriquement valides (245 au total)
    opcodes_valides_attendus = set(range(0x00, 0xFF + 1)) - OPCODES_INVALIDES_GB
    
    # Liste pour stocker TOUTES les occurrences d'opcodes valides trouvées
    opcodes_trouves = []

    try:
        with open(chemin_fichier, 'r', encoding='utf-8', errors='ignore') as f:
            contenu = f.read()
    except FileNotFoundError:
        print(f"Erreur : Le fichier '{chemin_fichier}' est introuvable.")
        sys.exit(1)

    # Regex adaptée pour Rust (gère 0x00, 0xFF_u8, b'\x00', etc.)
    pattern = r'\b0[xX]([0-9a-fA-F]{1,2})(?![0-9a-fA-F])|\\[xX]([0-9a-fA-F]{2})'
    matches = re.findall(pattern, contenu)

    for g1, g2 in matches:
        match = g1 if g1 else g2
        if match:
            valeur = int(match, 16)
            # On ne comptabilise que les opcodes qui font partie des valides attendus
            if valeur in opcodes_valides_attendus:
                opcodes_trouves.append(valeur)

    # Comptage des fréquences d'apparition
# Comptage des fréquences d'apparition
    compteur = Counter(opcodes_trouves)
    
    # Extraction des uniques et calcul des manquants
    uniques_presents = set(compteur.keys())
    manquants = opcodes_valides_attendus - uniques_presents
    
    # Identification des doublons (apparition > 1) -> CORRIGÉ ICI
    doublons = {opcode: nb for opcode, nb in compteur.items() if nb > 1}

    # --- AFFICHAGE DU RAPPORT ---
    print(f"Analyse du fichier Rust : {chemin_fichier}")
    print(f"Opcodes valides uniques détectés : {len(uniques_presents)} / 245")
    print(f"Opcodes invalides ignorés d'office : {len(OPCODES_INVALIDES_GB)}")
    print("==================================================")

    # 1. Section des Opcodes Manquants
    if not manquants:
        print("✅ Parfait ! Tous les 245 opcodes valides sont présents.")
    else:
        print(f"❌ Il te manque {len(manquants)} opcode(s) valide(s) :")
        for num in sorted(manquants):
            print(f"  - 0x{num:02X}")
                
    print("--------------------------------------------------")

    # 2. Section des Opcodes Doublés
    if not doublons:
        print("✅ Aucun opcode doublé détecté. Propre !")
    else:
        print(f"⚠️ Attention, {len(doublons)} opcode(s) apparaisse(nt) en double :")
        for num, nb in sorted(doublons.items()):
            print(f"  - 0x{num:02X} (trouvé {nb} fois)")

if __name__ == "__main__":
    if len(sys.argv) < 2:
        print("Usage: python check_opcodes.py <chemin_du_fichier.rs>")
        sys.exit(1)
        
    trouver_opcodes_manquants_et_doublons(sys.argv[1])