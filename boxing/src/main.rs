fn make_iterator(select: bool) -> Box<dyn Iterator<Item = u32>> {
    if select {
        Box::new([1u32].into_iter())
    } else {
        Box::new(vec![0u32].into_iter())
    }
}

fn use_iterator(iter: impl Iterator<Item = u32>) {
    for num in iter {
        println!("{}", num)
    }
}

fn use_dyn_iterator(iter: &mut dyn Iterator<Item = u32>) {
    for num in iter {
        println!("{}", num)
    }
}

struct MyVec<'a> {
    inner: Vec<&'a String>,
}

impl<'a> MyVec<'a> {
    fn iter(&self) -> Box<dyn Iterator<Item = &String> + '_> {
        Box::new(self.inner.iter().copied())
    }
}

fn main() {
    use_iterator(make_iterator(true));
    use_iterator([1u32].into_iter());

    use_dyn_iterator(&mut [1u32].into_iter());
    use_dyn_iterator(&mut make_iterator(true));

    let other = vec![2u32];
    let empty: Vec<u32>;
    let iter = if false {
        empty = Vec::new();
        empty.iter()
    } else {
        other.iter()
    };

    for item in iter {
        println!("{}", item)
    }
}
