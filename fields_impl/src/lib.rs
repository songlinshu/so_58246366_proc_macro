#[macro_use] extern crate derive_fields;

#[cfg(test)]
mod tests {

    use derive_fields::derivefields;

    #[derivefields(u32, "field", 3)]
    pub struct MyTest {
        foo: u32
    }

    #[test]
    fn has_fields() {
        let mut o = MyTest { 
            foo: 4,
            field_0: 1,
            field_1: 2,
            field_2: 3
        };
        o.field_0 = 3;
        assert_eq!(o.field_0, 3);
    }
}