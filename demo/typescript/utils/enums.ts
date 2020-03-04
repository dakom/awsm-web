//expects a regular enum, not one that overrides the assignment
//e.g. enum Foo { bar = "bar" } is NOT okay here, but enum Foo {bar} is
//
//takes an enum and gives back an array of Name,Index pairs
//get_enum_pairs :: Enum -> Array (Number, String)
export const get_enum_pairs = (target:any):Array<[number, string]> => 
    Object.keys(target)
        .map(index => Number(index))
        .filter(index => !isNaN(index))
        .map(index => ([index, target[index]]));

//converts a list of pair like Array<[number, string]> into a straight list of Array<string>
export const enum_pairs_to_list = (xs:Array<[number, string]>):Array<string> => 
    xs.reduce((acc, curr) => {
        acc[curr[0]] = curr[1];
        return acc;
    }, []);