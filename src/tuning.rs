
enum Tuning {
    StandardE,
    DropD,
    DropC,
    DropB,
    DroppedCsharp,
    StandardD,
    OpenD,
    OpenC,
}

impl Tuning {
    fn to_string(&self) -> &str {
        match self {
            Tuning::Standard => "eBGDAE",
            Tuning::DropD => "DADGBE",
            Tuning::DropC => "CGCFAD",
            Tuning::DropB => "BF#BEG#C#",
            Tuning::DroppedCsharp => "C#G#C#F#A#D#",
            Tuning::OpenD => "DADF#AD",
            Tuning::OpenC => "CGCGCE", 
        }
    }

    fn from_string(s: &str) -> Option<Tuning> {
        match s {
            "eBGDAE" => Some(Tuning::StandardE),
            "DADGBE" => Some(Tuning::DropD),
            "CGCFAD" => Some(Tuning::DropC),
            "BF#BEG#C#" => Some(Tuning::DropB),
            "C#G#C#F#A#D#" => Some(Tuning::DroppedCsharp),
            "DADF#AD" => Some(Tuning::OpenD),
            "CGCGCE" => Some(Tuning::OpenC),
            _ => None,
        }
    }
}