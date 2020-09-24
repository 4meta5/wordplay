//! Anagram generation
use std::collections::HashMap;
use std::hash::Hash;

pub struct Trie<K, V>
where
    K: Eq + Hash + Clone,
    V: Clone,
{
    val: Option<V>,
    kids: HashMap<K, Trie<K, V>>,
}

impl<K, V> Trie<K, V>
where
    K: Eq + Hash + Clone,
    V: Clone,
{
    fn new() -> Trie<K, V> {
        Trie {
            val: None,
            kids: HashMap::new(),
        }
    }

    fn insert_val(&mut self, v: V) -> bool {
        match self.val {
            Some(_) => return false,
            None => {
                self.val = Some(v);
                return true;
            }
        }
    }

    fn insert_trie(&mut self, path: Vec<K>, v: V) -> bool {
        if !path.is_empty() {
            self.kids
                .entry(path[0].clone())
                .or_insert(Trie::new())
                .insert_trie(path[1..].to_vec(), v);
            true
        } else {
            self.insert_val(v)
        }
    }

    fn val(&self) -> Option<V> {
        self.val.clone()
    }

    fn fetch(&self, path: Vec<K>) -> Option<V> {
        if !path.is_empty() {
            self.kids
                .get(&path[0])
                .and_then(|k| k.fetch(path[1..].to_vec()))
        } else {
            self.val()
        }
    }
}
