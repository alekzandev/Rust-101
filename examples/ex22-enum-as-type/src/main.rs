#[derive(Debug)]
enum WineRegions{
    Bordeaux,
    Burgundy,
    Champagne,
    Rhone,
    Alsace,
    Loire,
    Languedoc,
    Provence,
    Corsica,
    Jura,
    Savoie,
    SouthWest,
    Beaujolais,
    Other,
}

struct Wine {
    name: String,
    region: WineRegions,
    year: u16,
    price: f32,
}
fn supported_region(wr: WineRegions){
    match wr {
        WineRegions::Bordeaux => println!("Bordeaux"),
        WineRegions::Burgundy => println!("Burgundy"),
        WineRegions::Champagne => println!("Champagne"),
        WineRegions::Rhone => println!("Rhone"),
        WineRegions::Alsace => println!("Alsace"),
        WineRegions::Loire => println!("Loire"),
        WineRegions::Languedoc => println!("Languedoc"),
        WineRegions::Provence => println!("Provence"),
        WineRegions::Corsica => println!("Corsica"),
        WineRegions::Jura => println!("Jura"),
        WineRegions::Savoie => println!("Savoie"),
        WineRegions::SouthWest => println!("SouthWest"),
        WineRegions::Beaujolais => println!("Beaujolais"),
        _ => println!("Unknown region"),
    }
}

fn get_popularity(wr: &WineRegions) -> u8 {
    match wr {
        WineRegions::Bordeaux => 10,
        WineRegions::Burgundy => 9,
        WineRegions::Champagne => 8,
        WineRegions::Rhone => 7,
        WineRegions::Alsace => 6,
        WineRegions::Loire => 5,
        WineRegions::Languedoc => 4,
        WineRegions::Provence => 3,
        WineRegions::Corsica => 2,
        WineRegions::Jura => 1,
        WineRegions::Savoie => 1,
        WineRegions::SouthWest => 1,
        WineRegions::Beaujolais => 1,
        _ => 0,
    }

}

fn main() {
    let wine1 = Wine {
        name: String::from("Chateau Margaux"),
        region: WineRegions::Provence,
        year: 2012,
        price: 7000.0,
    };

    let wine2 = Wine {
        name: String::from("Bourgogne"),
        region: WineRegions::Bordeaux,
        year: 2014,
        price: 8000.0,
    };

    let wine3 = Wine {
        name: String::from("Champagne"),
        region: WineRegions::Champagne,
        year: 2015,
        price: 1000.0,
    };

    println!("Wine 1: {} from {:?} of year {} costs ${}", wine1.name, wine1.region, wine1.year, wine1.price);
    println!("Wine 2: {} from {:?} of year {} costs ${}", wine2.name, wine2.region, wine2.year, wine2.price);
    
    supported_region(wine1.region);
    supported_region(wine2.region);
    supported_region(WineRegions::Other);
    
    println!("Popularity of {:?} is {}", wine3.region, get_popularity(&wine3.region));
    println!("Popularity of {:?} is {}", WineRegions::Other, get_popularity(&WineRegions::Other));
}
