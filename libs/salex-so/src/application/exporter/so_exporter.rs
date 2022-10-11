use crate::application::queries::GetBForms;
use crate::application::repositories::SoOrdRepository;
use crate::domain::entities::SoOrd;
use html::Html;
use html::{Dom, Element, Node};
use std::error::Error;
use std::io;
use std::sync::Arc;

pub struct SoExporter {
    so_ord_repo: Arc<dyn SoOrdRepository>,
    get_bforms: Arc<dyn GetBForms>,
}

impl SoExporter {
    pub fn new(so_ord_repo: Arc<dyn SoOrdRepository>, get_bforms: Arc<dyn GetBForms>) -> Self {
        Self {
            so_ord_repo,
            get_bforms,
        }
    }
}

impl SoExporter {
    pub async fn write_so_underlag<'a>(
        &self,
        mut writer: Box<dyn io::Write + 'a>,
        ord_idx: &[u32], // , ord_ix: Box<dyn Iterator<Item=u32>>
    ) -> Result<(), Box<dyn std::error::Error>> {
        writer.write(b"<!DOCTYPE html[\n")?;
        writer.write(b"    <!DOCTYPE html[\n")?;
        writeln!(writer, r#"    <!ENTITY ouml  "&#246;">"#)?;
        writeln!(writer, r#"    <!ENTITY Ouml "&#214;">"#)?;
        writeln!(writer, r#"    <!ENTITY auml "&#228;">"#)?;
        writeln!(writer, r#"    <!ENTITY nbsp "&#160;">"#)?;
        writeln!(writer, r#"    <!ENTITY ndash "&#8211;">"#)?;
        writeln!(writer, r#"    <!ENTITY mdash "&#8212;">"#)?;
        writeln!(writer, r#"    <!ENTITY plus "&#43;">"#)?;
        writeln!(writer, r#"    <!ENTITY colon "&#58;">"#)?;
        writeln!(writer, r#"    <!ENTITY ast "&#42;">"#)?;
        writeln!(writer, r#"    <!ENTITY amp "&#38;">"#)?;
        writeln!(writer, r#"    <!ENTITY lt "&#60;">"#)?;
        writeln!(writer, r#"    <!ENTITY gt "&#62;">"#)?;
        writer.write(b"    ]>\n")?;
        writer.write(b"<html>\n")?;
        self.skriv_head(&mut writer)?;
        writer.write(b"<body>\n")?;
        writeln!(writer, r#"<div class="lemmalista">"#)?;
        for idx in ord_idx {
            tracing::debug!("idx={}", idx);
            let lemma = self.so_ord_repo.get(*idx).await;
            tracing::debug!("lemma.ortografi={}", &lemma.ortografi);
            let adds_str = html::to_string(&self.skriv_adds(&lemma).await);
            writeln!(writer, "{}", adds_str)?;
        }
        writeln!(writer, "</div>")?;
        writer.write(b"</body>\n")?;
        writer.write(b"</html>\n")?;
        Ok(())
    }

    fn skriv_head<'a>(
        &self,
        writer: &mut Box<dyn io::Write + 'a>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        writer.write(br#"<head><meta http-equiv="Content-Type" content="text/html; charset=UTF-8"></meta><title>Lista ord eller artiklar</title></head>"#)?;
        Ok(())
    }

    async fn skriv_adds(&self, lemma: &SoOrd) -> Element {
        let mut div_adds = Html::div().class("adds");
        div_adds = div_adds.child(
            Html::span()
                .class("tillsnr")
                .text(format!("{}", lemma.s_nr))
                .build(),
        );
        let mut div_lemma_form = Html::div();
        if !lemma.lemmaundertyp.is_empty() {
            let span_wtype = Html::span()
                .class("wtype")
                .text(&lemma.lemmaundertyp)
                .build();
            div_lemma_form = div_lemma_form.child(span_wtype);
        }
        div_lemma_form = div_lemma_form.child(
            Html::span()
                .class("alfasort")
                .text(&lemma.sorteringsform)
                .build(),
        );
        if self.so_ord_repo.has_heteronym(lemma).await {
            div_lemma_form = div_lemma_form.child(
                Html::span()
                    .class("show")
                    .child(Html::sup().text(format!("{}", lemma.lm_sabob)).build())
                    .text(&lemma.ortografi)
                    .build(),
            )
        } else {
            div_lemma_form =
                div_lemma_form.child(Html::span().class("show").text(&lemma.ortografi).build());
        }
        div_adds = div_adds.child(div_lemma_form.build());

        let mut sorted_lemma_refs = lemma
            .lemma_referenser
            .clone()
            .sort_by(|a, b| a.lemmatyp.cmp(&b.lemmatyp));
        dbg!(sorted_lemma_refs);

        // böjningsformer
        for bform in self
            .get_bforms
            .query(lemma.s_nr, &lemma.böjningsklass)
            .await
        {
            let reflexiv_form = format!("{} sig", &bform.sortform);
            let partikelfilter = bform.tagg == "V0N0A" && &lemma.böjningsklass[..1] != "M";
            let reflexiv = bform.tagg == "V0N0A" && &lemma.ortografi == &reflexiv_form;

            if !partikelfilter && !reflexiv {
                div_adds = div_adds.child(
                    Html::div()
                        .child(Html::span().class("wtype").text("bform").build())
                        .build(),
                );
            }
        }
        div_adds.build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[rstest]
    fn export_superlemmalista() {
        // let so_exporter = SoExporter::new();

        // let mut buf = Vec::new();

        // {
        //     let writer = Box::new(&mut buf);
        //     so_exporter.write_so_underlag(writer).unwrap();
        // }
        // let output = std::str::from_utf8(buf.as_slice()).unwrap();
        // let facit =
        // assert_eq!(
        //     output, r#"<!DOCTYPE html>"#);
    }
}
