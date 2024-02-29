const ORDKLASS_SALEX: [(&str, &str); 16] = [
    ("adj.", "adjektiv"),
    ("adv.", "adverb"),
    ("i sms.", "i sammansättningar"),
    ("interj.", "interjektion"),
    ("konj.", "konjunktion"),
    ("prep.", "preposition"),
    ("pron.", "pronomen"),
    ("räkn.", "räkneord"),
    ("s. pl.", "substantiv i plural"),
    ("subj.", "subjunktion"),
    ("s.", "substantiv"),
    ("v.", "verb"),
    ("s. oböjl.", "substantiv ingen böjning"),
    ("Hänvisningslemman", "hänvisningslemman"),
    ("subst.", "substantiv"),
    ("adjektivisk slutled", "adjektiviskt slutled"),
    // # s. pl.   (673)	s. pl.  550 	substantiv i plural
    // # 	s. pl.  1
    // # s. pl. best.   (18)	s. pl. best.    12  	s. pl. best.
    // # subj.   (57)	subj.   42  	subjunktion
    // # 	s.  1   	GRANSKA
    // # subst.   (115851)	s.  94066   	substantiv
    // # substantiverat adj.   (35)		substantiverat adj.
    // # substantiviskt förled   (1)		substantiviskt förled
    // # substantiviskt slutled   (39)	substantiviskt slutled  13  	substantiviskt slutled
    // # superl. adj.   (1)		GRANSKA
    // # superl. adv.   (1)		GRANSKA
    // # v. dep.   (89)		GRANSKA
    // # v., pres.   (1)	v., pres.   1   	v., pres.
    // # v., pret.   (1)	v., pret.   1   	v., pret.
    // # verb   (25628)	v.  8610    	verb
    // # verbalt slutled   (1)	verbalt slutled 1	verbalt slutled
    // # ** s. **   (1)		GRANSKA
    // # ?   (5)		GRANSKA
    // # ??*??   (11)		GRANSKA
    // # 	Hänvisningslemman 504	hänvisningslemman
];

pub fn expand_ordklass(ordklass: &str) -> &str {
    for (old_ordklass, salex_ordklass) in ORDKLASS_SALEX.iter() {
        if ordklass == *old_ordklass {
            return salex_ordklass;
        }
    }
    ordklass
}

pub fn create_key(ortografi: &str, ordklass: &str, böjningsklass: &str) -> String {
    let ortografi = ortografi.replace(".", "");
    let ortografi = diacritics::remove_diacritics(&ortografi);
    let ordklass = ordklass.replace(".", "");
    format!("{}..{}..{}", ortografi, ordklass, böjningsklass)
        .replace(" ", "_")
        .to_lowercase()
}
