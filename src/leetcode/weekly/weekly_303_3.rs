use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::iter::FromIterator;
use crate::leetcode::Solution;

#[test]
fn weekly_303_3_test() {
    let mut ratings = FoodRatings::new(
        vec!["kimchi", "miso", "sushi", "moussaka", "ramen", "bulgogi"].iter().map(|&x| x.to_string()).collect(),
        vec!["korean", "japanese", "japanese", "greek", "japanese", "korean"].iter().map(|&x| x.to_string()).collect(),
        vec![9, 12, 8, 15, 14, 7]
    );
    assert_eq!(ratings.highest_rated("korean".to_string()), "kimchi");
    assert_eq!(ratings.highest_rated("japanese".to_string()), "ramen");
    ratings.change_rating("sushi".to_string(), 16);
    assert_eq!(ratings.highest_rated("japanese".to_string()), "sushi");
    ratings.change_rating("ramen".to_string(), 16);
    assert_eq!(ratings.highest_rated("japanese".to_string()), "ramen");
}

#[derive(Eq, PartialEq)]
struct Food {
    name: String,
    cuisine: String,
    rating: i32
}

impl Clone for Food {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            cuisine: self.cuisine.clone(),
            rating: self.rating
        }
    }
}

impl PartialOrd<Self> for Food {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.rating.cmp(&self.rating).then(self.name.cmp(&other.name)))
    }
}

impl Ord for Food {
    fn cmp(&self, other: &Self) -> Ordering {
        other.rating.cmp(&self.rating).then(self.name.cmp(&other.name))
    }
}

struct FoodRatings {
    idx: HashMap<String, Food>,
    foods: HashMap<String, BTreeSet<Food>>
}

impl FoodRatings {

    fn new(foods: Vec<String>, cuisines: Vec<String>, ratings: Vec<i32>) -> Self {
        let foods: Vec<_> = foods.iter().zip(cuisines.iter().zip(ratings.iter())).map(|(name, (cuisine, &rating))| {
            Food {
                name: name.clone(),
                cuisine: cuisine.clone(),
                rating
            }
        }).collect();
        let idx: HashMap<String, Food> = HashMap::from_iter(foods.iter().map(|x| (x.name.clone(), x.clone())));
        let foods: HashMap<String, BTreeSet<Food>> = foods.into_iter().fold(HashMap::new(), |mut r, x| {
            let entry = r.entry(x.cuisine.clone()).or_insert(BTreeSet::new());
            (*entry).insert(x);
            r
        });
        Self {
            idx,
            foods
        }
    }

    fn change_rating(&mut self, food: String, new_rating: i32) {
        let f = self.idx.get(&food);
        if f.is_none() {
            return
        }
        let mut f = f.unwrap().clone();
        let entry = self.foods.entry(f.cuisine.clone()).or_insert(BTreeSet::new());
        (*entry).remove(&f);

        f.rating = new_rating;
        self.idx.insert(f.name.clone(), f.clone());
        (*entry).insert(f.clone());
    }

    fn highest_rated(&self, cuisine: String) -> String {
        if let Some(foods) = self.foods.get(cuisine.as_str()) {
            for f in foods {
                return f.name.clone()
            }
            return "".to_string()
        } else {
            "".to_string()
        }
    }
}
