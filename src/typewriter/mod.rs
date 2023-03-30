use egui::{Ui, RichText};

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TypeState {
    input: String,
    challenge: String,
}

impl Default for TypeState {
    fn default() -> Self {
        Self {
            input: Default::default(),
            challenge: Default::default(),
        }
    }
}

pub trait Challenge {
   fn into_challenge(&self) -> String;
}



impl TypeState {
    pub fn render(&mut self, ui: &mut Ui, provider: &(impl Challenge + ?Sized)) {
        //win condition
        if self.input.eq(&self.challenge) {
            self.challenge.clear();
            self.input.clear();
            self.challenge = provider.into_challenge();
        }

        let challenge_text = RichText::new(self.challenge.to_string()).size(90.0);
        ui.heading(challenge_text);
        
        let input_text = RichText::new(self.input.to_string()).size(90.0);
        ui.heading(input_text);
        
        ui.text_edit_multiline(&mut self.input);

        ui.label("");
        ui.label("");
        ui.label("");
        ui.label("");
        ui.label("");
        ui.label("");
        ui.label("");
        ui.label("");
        ui.label("");
        ui.label("");
        ui.label("");
        ui.label("");
        ui.label("");
        ui.label("");

    

    }
}


