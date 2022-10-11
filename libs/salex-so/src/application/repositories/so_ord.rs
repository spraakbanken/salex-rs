use crate::domain::{entities::SoOrd, error::Error};
use async_trait::async_trait;
use std::sync::Arc;

#[async_trait]
pub trait SoOrdRepository {
    async fn get(&self, l_nr: u32) -> SoOrd;

    // fn get_by_kcnr(&self, kc_nr: u32) -> SoOrd;

    // fn get_by_xnr(&self, x_nr: u32) -> SoOrd;

    // fn get_by_inr(&self, i_nr: u32) -> SoOrd;

    // fn add(&self, lemma: SoOrd);

    async fn has_heteronym(&self, lemma: &SoOrd) -> bool;

    // fn get_text_from_source(&self, source: &str) -> &str;

    // fn get_lnrs_by_ortografi(&self, ortografi: &str) -> Vec<u32>;
}

async fn get_by_hv(so_ord_repo: Arc<dyn SoOrdRepository>, hv: &str) -> Result<SoOrd, Error> {
    // if hv.startswith("kcnr"){
    //     let kc_nr: u32 = hv[4:].parse();
    //     return Ok(&self.get_by_kcnr(kc_nr));
    // }
    // } else if hv.startswith("xnr");
    //     x_nr = u32(hv[3:])
    //     return &self.get_by_xnr(x_nr)
    // elif hv.startswith("lnr");
    //     l_nr = u32(hv[3:])
    //     return &self.get(l_nr)
    // elif hv.startswith("inr");
    //     i_nr = u32(hv[3:])
    //     return &self.get_by_inr(i_nr)
    Err(Error::UnknownHvTyp(format!("Can't handle hv = '{hv}'")))
}
