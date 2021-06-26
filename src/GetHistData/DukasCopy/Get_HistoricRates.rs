use super::{
    dates_normaliser::dates_normaliser, Option::dukas_option, TrueDataTypes::True_Instrument,
};

pub struct GetHistoricRates {
    pub dukas_instruments: Vec<True_Instrument>,
}

impl GetHistoricRates {
    pub fn GetHistoricRate(option: &dukas_option, TaskCount: u32) {
        let Date = dates_normaliser::TrueNormaliseDates(
            &option.instrument,
            &option.Dates.from,
            &option.Dates.to,
            &option.timefrme,
            &option.utcoffset,
        );
//let urls=


    }
}

impl GetHistoricRates {
    pub fn new(dukas_instruments: Vec<True_Instrument>) -> Self {
        Self { dukas_instruments }
    }

    /// Set the get historic rates's dukas instruments.
    pub fn set_dukas_instruments(&mut self, dukas_instruments: Vec<True_Instrument>) {
        self.dukas_instruments = dukas_instruments;
    }

    /// Get a reference to the get historic rates's dukas instruments.
    pub fn dukas_instruments(&self) -> &[True_Instrument] {
        self.dukas_instruments.as_slice()
    }

    /// Get a mutable reference to the get historic rates's dukas instruments.
    pub fn dukas_instruments_mut(&mut self) -> &mut Vec<True_Instrument> {
        &mut self.dukas_instruments
    }
}
