package zio.schema.toon

import zio.schema._
import zio.schema.codec.BinaryCodec
import java.nio.ByteBuffer
import java.io.{OutputStreamWriter, StringReader}
import java.nio.charset.StandardCharsets

/**
 * Abstract codec for TOON encoding/decoding.
 */
abstract class ToonBinaryCodec[A](val valueType: Int = ToonBinaryCodec.objectType)
    extends BinaryCodec[A] {

  def decodeValue(in: ToonReader, default: A): A
  def encodeValue(x: A, out: ToonWriter): Unit

  override def decode(input: ByteBuffer): Either[SchemaError, A] =
    decode(input, ToonReaderConfig)

  override def encode(value: A, output: ByteBuffer): Unit =
    encode(value, output, ToonWriterConfig)

  def decode(input: ByteBuffer, config: ToonReaderConfig): Either[SchemaError, A] = {
       val bytes = new Array[Byte](input.remaining())
       input.get(bytes)
       val s = new String(bytes, StandardCharsets.UTF_8)
       val reader = new ToonReader(new StringReader(s), config)
       try Right(decodeValue(reader, null.asInstanceOf[A]))
       catch { case e: Exception => Left(SchemaError.ReadError(Cause.Fail(e.getMessage))) }
  }

  def encode(value: A, output: ByteBuffer, config: ToonWriterConfig): Unit = {
     val stream = new ByteBufferOutputStream(output)
     val writer = new ToonWriter(new OutputStreamWriter(stream, StandardCharsets.UTF_8), config)
     encodeValue(value, writer)
     writer.flush()
  }

  // Helper class
  protected class ByteBufferOutputStream(buf: java.nio.ByteBuffer) extends java.io.OutputStream {
      def write(b: Int): Unit = buf.put(b.toByte)
      override def write(bytes: Array[Byte], off: Int, len: Int): Unit = buf.put(bytes, off, len)
  }
}

object ToonBinaryCodec {
  val objectType  = 0
  val primitiveType = 1
}

/**
 * Specifies how arrays should be encoded in TOON format.
 */
sealed trait ArrayFormat
object ArrayFormat {
  case object Auto extends ArrayFormat
  case object Tabular extends ArrayFormat
  case object Inline extends ArrayFormat
  case object List extends ArrayFormat
}

/**
 * Configuration for ToonReader.
 */
class ToonReaderConfig(
  val preferredBufSize: Int = 32768,
  val strictArrayLength: Boolean = true
)

object ToonReaderConfig extends ToonReaderConfig()

/**
 * Configuration for ToonWriter.
 */
class ToonWriterConfig(
  val indentSize: Int = 2,
  val lineEnding: String = "\n"
)

object ToonWriterConfig extends ToonWriterConfig()
