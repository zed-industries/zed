package zio.schema.toon

import zio.schema._
import java.io.{OutputStreamWriter, StringReader}
import java.nio.ByteBuffer

// ... (MetaSchema, ToonBinaryCodecDeriver singleton removed/kept as is)

class ToonBinaryCodecDeriver {

  // ... (derivePrimitive, deriveRecord same as before) ...
  
  // Need to restate the class to edit deriveSequence
  
  // --- Primitives ---
  def derivePrimitive[A](standardType: StandardType[A]): ToonBinaryCodec[A] = {
    new ToonBinaryCodec[A](ToonBinaryCodec.primitiveType) {
       override def decodeValue(in: ToonReader, default: A): A = {
           val s = in.readString()
           (standardType match {
               case StandardType.StringType => s
               case StandardType.BoolType => s.toBoolean
               case StandardType.IntType => s.toInt
               case StandardType.LongType => s.toLong
               case StandardType.FloatType => s.toFloat
               case StandardType.DoubleType => s.toDouble
               case StandardType.ShortType => s.toShort
               case StandardType.ByteType => s.toByte
               case StandardType.CharType => if (s.nonEmpty) s.charAt(0) else throw new RuntimeException("Empty char")
               case _ => throw new RuntimeException(s"Unsupported primitive type: $standardType")
           }).asInstanceOf[A]
       }
       override def encodeValue(x: A, out: ToonWriter): Unit = {
         out.writeRawString(x.toString)
       }
    }
  }

  // --- Records ---
  def deriveRecord[A](structure: Schema.Record[A]): ToonBinaryCodec[A] = {
    new ToonBinaryCodec[A](ToonBinaryCodec.objectType) {
      val fields = structure.fields
      override def encodeValue(x: A, out: ToonWriter): Unit = {
        fields.foreach { field =>
            val value = field.get(x)
            out.writeKey(field.name)
            val fieldCodec = recursiveDerive(field.schema).asInstanceOf[ToonBinaryCodec[Any]]
             if (fieldCodec.valueType == ToonBinaryCodec.objectType) {
                out.newLine()
                out.indent()
                fieldCodec.encodeValue(value, out)
                out.unindent()
              } else {
                out.writeSpace()
                fieldCodec.encodeValue(value, out)
                out.newLine()
              }
        }
      }
      override def decodeValue(in: ToonReader, default: A): A = {
          val baseIndent = in.peekIndent()
          val values = scala.collection.mutable.Map[String, Any]()
          while (in.peekIndent() >= baseIndent && in.peekLine() != null) {
              val (key, inlineValue) = in.readField(baseIndent)
              val fieldOpt = fields.find(_.name == key)
              if (fieldOpt.isDefined) {
                  val field = fieldOpt.get
                  val codec = recursiveDerive(field.schema).asInstanceOf[ToonBinaryCodec[Any]]
                  val decoded = if (inlineValue.isDefined) {
                      val tmpReader = new ToonReader(new StringReader(inlineValue.get), ToonReaderConfig)
                      codec.decodeValue(tmpReader, null)
                  } else {
                      codec.decodeValue(in, null)
                  }
                  values(key) = decoded
              }
          }
          val args = zio.Chunk.fromIterable(fields.map { f => values.getOrElse(f.name, throw new RuntimeException(s"Missing field ${f.name}")) })
          structure.construct(args).fold(e => throw new RuntimeException(e), v => v)
      }
    }
  }

  // --- Sequences ---
  def deriveSequence[A](schema: Schema.Sequence[_, A, _]): ToonBinaryCodec[Any] = {
    new ToonBinaryCodec[Any](ToonBinaryCodec.objectType) {
        val elementCodec = recursiveDerive(schema.elementSchema).asInstanceOf[ToonBinaryCodec[Any]]
        
        override def encodeValue(xs: Any, out: ToonWriter): Unit = {
            val chunk = schema.toChunk(xs.asInstanceOf[schema.Col])
            if (chunk.isEmpty) return 
            
            val isPrimitive = elementCodec.valueType != ToonBinaryCodec.objectType
            
            if (isPrimitive) {
                // Inline CSV for primitives
                chunk.zipWithIndex.foreach { case (item, idx) =>
                    if (idx > 0) out.writeComma()
                    elementCodec.encodeValue(item, out)
                }
            } else {
                // List format for objects
                chunk.foreach { item =>
                    out.writeRawString("- ")
                    // Force newline for object content to ensure clean indentation
                    // This matches the "List Item Block" style
                    out.newLine() 
                    out.indent() // Indent for the object body
                    elementCodec.encodeValue(item, out) 
                    out.unindent()
                }
            }
        }

        override def decodeValue(in: ToonReader, default: Any): Any = {
            val buffer = zio.ChunkBuilder.make[Any]()
            
            // Check for Inline List (Primitives)
            val line = in.peekLine()
            if (line != null && !line.startsWith("- ")) {
                // Assume Inline CSV
                val items = in.readInlineList()
                items.foreach { itemStr =>
                    val tmpReader = new ToonReader(new StringReader(itemStr), ToonReaderConfig)
                    buffer += elementCodec.decodeValue(tmpReader, null)
                }
                return schema.fromChunk(buffer.result())
            }
            
            // Assume Block List (Objects)
            val baseIndent = in.peekIndent()
            while (in.peekIndent() >= baseIndent && in.peekLine() != null) {
                val currentLineStr = in.peekLine()
                if (currentLineStr.trim.startsWith("- ")) {
                    in.consumeLine() // Consume marker
                    // Since we forced Newline + Indent in Encoder, 
                    // the object fields are just naturally following at higher indent.
                    // We just call decodeValue.
                    buffer += elementCodec.decodeValue(in, null)
                } else {
                   // End of list
                   return schema.fromChunk(buffer.result())
                }
            }
            schema.fromChunk(buffer.result())
        }
    }
  }

  def recursiveDerive[A](schema: Schema[A]): ToonBinaryCodec[A] = {
      schema match {
          case s: Schema.Primitive[A] => derivePrimitive(s.standardType)
          case s: Schema.Record[A] => deriveRecord(s)
          case s: Schema.Sequence[_, _, _] => deriveSequence(s).asInstanceOf[ToonBinaryCodec[A]]
          case _ => throw new RuntimeException("Unsupported schema type for V1 demo")
      }
  }
}
