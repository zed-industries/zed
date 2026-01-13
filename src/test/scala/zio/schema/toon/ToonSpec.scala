package zio.schema.toon

import zio.test._
import zio.test.Assertion._
import zio.schema._
import zio._ // Import essential ZIO types including Chunk
import java.nio.ByteBuffer

object ToonSpec extends ZIOSpecDefault {

  // Define a test model
  case class Person(name: String, age: Int)
  
  val personSchema: Schema[Person] = Schema.CaseClass2[String, Int, Person](
    TypeId.parse("Person"),
    Schema.Field("name", Schema.primitive(StandardType.StringType), get0 = _.name, set0 = (p, v) => p.copy(name = v)),
    Schema.Field("age", Schema.primitive(StandardType.IntType), get0 = _.age, set0 = (p, v) => p.copy(age = v)),
    (name, age) => Person(name, age)
  )

  val deriver = new ToonBinaryCodecDeriver()
  val personCodec = deriver.recursiveDerive(personSchema)

  override def spec = suite("ToonSpec")(
    test("Round-trip encoding/decoding of Person") {
      val alice = Person("Alice", 30)
      val buffer = ByteBuffer.allocate(1024)
      personCodec.encode(alice, buffer)
      buffer.flip()
      val bytes = new Array[Byte](buffer.remaining())
      buffer.get(bytes)
      val outputString = new String(bytes, "UTF-8")
      buffer.rewind()
      val result = personCodec.decode(buffer)
      assert(outputString)(containsString("name: Alice")) &&
      assert(result)(isRight(equalTo(alice)))
    },
    test("Nested Object Round-trip") {
        case class Employee(person: Person, role: String)
        val empSchema = Schema.CaseClass2[Person, String, Employee](
            TypeId.parse("Employee"),
            Schema.Field("person", personSchema, get0 = _.person, set0 = (e, v) => e.copy(person = v)),
            Schema.Field("role", Schema.primitive(StandardType.StringType), get0 = _.role, set0 = (e, v) => e.copy(role = v)),
            (p, r) => Employee(p, r)
        )
        val empCodec = deriver.recursiveDerive(empSchema)
        val emp = Employee(Person("Bob", 40), "Developer")
        val buffer = ByteBuffer.allocate(1024)
        empCodec.encode(emp, buffer)
        buffer.flip()
        buffer.rewind()
        val result = empCodec.decode(buffer)
        assert(result)(isRight(equalTo(emp)))
    },
    test("Sequence of Primitives") {
        val seqSchema = Schema.Sequence(Schema.primitive(StandardType.IntType), _.asInstanceOf[Chunk[Int]], (c: Chunk[Int]) => c, Chunk.empty)
        val codec = deriver.recursiveDerive(seqSchema)
        val input = Chunk(1, 2, 3, 4, 5)
        
        val buffer = ByteBuffer.allocate(1024)
        codec.encode(input, buffer)
        buffer.flip()
        val bytes = new Array[Byte](buffer.remaining())
        val str = new String(bytes, "UTF-8")
        
        buffer.rewind()
        val result = codec.decode(buffer)
        
        assert(str)(equalTo("1,2,3,4,5")) &&
        assert(result)(isRight(equalTo(input)))
    },
    test("Sequence of Objects") {
        val seqSchema = Schema.Sequence(personSchema, _.asInstanceOf[Chunk[Person]], (c: Chunk[Person]) => c, Chunk.empty)
        val codec = deriver.recursiveDerive(seqSchema)
        val input = Chunk(Person("Alice", 30), Person("Bob", 40))
        
        val buffer = ByteBuffer.allocate(1024)
        codec.encode(input, buffer)
        buffer.flip()
        
        buffer.rewind()
        val result = codec.decode(buffer)
        assert(result)(isRight(equalTo(input)))
    }
  )
}
