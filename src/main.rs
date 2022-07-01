extern crate minheap;
use minheap::MinHeap;


fn main() {
    println!("Hello, world!");


    let mut v = MinHeap::<i32>::new();
    v.insert(1,61);
    v.insert(2,-60);
    v.insert(3,50);
    v.insert(4,-10);
    v.insert(5,18);
    v.insert(6,40);

    assert!(v.validate_heap())

}


#[cfg(test)]
mod tests {

    #[test]
    fn test1() {
        use crate::minheap::MinHeap;

        let mut v = MinHeap::<u32>::new();
        v.insert(100,61);
        v.insert(200,60);
        v.insert(300,50);
        v.insert(400,10);
        v.insert(500,18);
        v.insert(600,40);
        assert!(v.validate_heap())
    } 

    #[test]
    fn test2() {
        use crate::minheap::MinHeap;
        //#[derive(Debug,PartialOrd,PartialEq)]
        #[derive(Debug)]
        struct Person {
            rank: u32,
            age: u32,
            name: String,
        }

        use std::cmp::Ordering;

        impl PartialOrd for Person {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                self.age.partial_cmp(&other.age)
            }
        }

        impl PartialEq for Person {
            fn eq(&self, other: &Self) -> bool {
                self.age == other.age
            }
        }

        let mut v = MinHeap::<Person>::new();
        v.insert(10,Person { name: "Marvin".to_string(), age:61,  rank: 1});
        v.insert(20,Person { name: "Marvin".to_string(), age:60,  rank: 2});
        v.insert(30,Person { name: "Marcia".to_string(), age:50,  rank: 2});
        v.insert(40,Person { name: "Jordana".to_string(), age:10,  rank: 3});
        v.insert(50,Person { name: "Gizmo".to_string(), age:18,  rank:4 });
        assert!(v.validate_heap());
        assert_eq!(v.get_min().unwrap().age,10);
        assert_eq!(v.get_min().unwrap().age,18);
        assert_eq!(v.get_min().unwrap().age,50);

    }

    #[test]
    fn test3() {
        use crate::minheap::MinHeap;

        let mut v = MinHeap::<i32>::new();
        v.insert(1,-10);
        v.insert(2,5);
        v.insert(3,-1);
        v.insert(4,3);


        assert_eq!(v.get_min(),Some(-10));
        assert_eq!(v.get_min(),Some(-1));
        assert_eq!(v.get_min(),Some(3));
        assert_eq!(v.get_min(),Some(5));

    }

    #[test]
    fn test4_enum() {
        use crate::minheap::MinHeap;

        #[derive(Debug)]
        struct SimpleInfo {
            id: u32,
            weight: u32,
        }

        #[derive(Debug)]
        struct CompoundInfo {
            id: u32,
            weight: u32,
            low: u32,
            high: u32,

        }

        #[derive(Debug)]
        enum TestSort {
            Simple(SimpleInfo),
            Compound(CompoundInfo)
        }

        impl TestSort {
            pub fn get_weight(self) -> u32 {
                match self {
                    Simple(a) => a.weight,
                    Compound(a) => a.weight,
                }

            }
        }

        use std::cmp::Ordering;

        impl Ord for TestSort {
            fn cmp(&self, other: &Self) -> Ordering {

                match (self, other) {
                    (Simple(a), Simple(b)) =>  a.weight.cmp(&b.weight),
                    (Simple(a), Compound(b)) =>  a.weight.cmp(&b.weight),
                    (Compound(a), Simple(b)) =>  a.weight.cmp(&b.weight),
                    (Compound(a), Compound(b)) =>  a.weight.cmp(&b.weight),
                    _ => Ordering::Less
                }
            }
        }

        impl PartialOrd for TestSort {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {

                match (self, other) {
                    (Simple(a), Simple(b)) =>  a.weight.partial_cmp(&b.weight),
                    (Simple(a), Compound(b)) =>  a.weight.partial_cmp(&b.weight),
                    (Compound(a), Simple(b)) =>  a.weight.partial_cmp(&b.weight),
                    (Compound(a), Compound(b)) =>  a.weight.partial_cmp(&b.weight),
                    _ => Some(Ordering::Less)
                }
            }
        }

        impl PartialEq for TestSort {
            fn eq(&self, other: &Self) -> bool {
                match (self, other) {
                    (Simple(a), Simple(b)) =>  a.weight == b.weight,
                    (Simple(a), Compound(b)) =>  a.weight == b.weight,
                    (Compound(a), Simple(b)) =>  a.weight == b.weight,
                    (Compound(a), Compound(b)) =>  a.weight == b.weight,
                    _ => false
                }
            }
        }

        impl Eq for TestSort { }

        use TestSort::Simple;
        use TestSort::Compound;

        let mut v = MinHeap::<TestSort>::new();
        let mut id=1;
        v.insert(id,Simple(SimpleInfo {id: id,weight: 1}));   id +=1;
        v.insert(id,Simple(SimpleInfo {id: id,weight: 3})); id +=1;
        v.insert(id,Simple(SimpleInfo {id: id,weight: 5})); id +=1;
        v.insert(id,Compound(CompoundInfo {id: id,weight: 6,low: 1, high: 2})); id +=1;
        v.insert(id,Compound(CompoundInfo {id: id,weight: 20,low: 1, high: 2})); id +=1;
        v.insert(id,Compound(CompoundInfo {id: id,weight: 1,low: 1, high: 2})); id +=1;
        v.insert(id,Compound(CompoundInfo {id: id,weight: 2,low: 1, high: 2})); id +=1;
        v.insert(id,Simple(SimpleInfo {id: 2,weight: 8})); id +=1;
        assert!(v.validate_heap());
        assert_eq!(v.get_min().unwrap().get_weight(),1);
        assert_eq!(v.get_min().unwrap().get_weight(),1);
        assert_eq!(v.get_min().unwrap().get_weight(),2);
        assert_eq!(v.get_min().unwrap().get_weight(),3);
        assert_eq!(v.get_min().unwrap().get_weight(),5);
        assert_eq!(v.get_min().unwrap().get_weight(),6);
        assert_eq!(v.get_min().unwrap().get_weight(),8);
        assert_eq!(v.get_min().unwrap().get_weight(),20);

    }



}
