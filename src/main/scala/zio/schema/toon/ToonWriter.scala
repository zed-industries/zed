package zio.schema.toon

import java.io.Writer

class ToonWriter(writer: Writer, config: ToonWriterConfig) {
  private var indentationLevel = 0
  private var isStartOfLine = true

  def writeRawString(s: String): Unit = {
    ensureIndentation()
    writer.write(s)
  }
  
  def writeKey(key: String): Unit = {
      ensureIndentation()
      writer.write(key)
      writer.write(":")
  }

  def writeQuotedString(s: String): Unit = {
    ensureIndentation()
    writer.write('"')
    s.foreach {
      case '\\' => writer.write("\\\\")
      case '"'  => writer.write("\\\"")
      case '\n' => writer.write("\\n")
      case '\r' => writer.write("\\r")
      case '\t' => writer.write("\\t")
      case c    => writer.write(c)
    }
    writer.write('"')
  }

  def indent(): Unit = indentationLevel += 1
  def unindent(): Unit = if (indentationLevel > 0) indentationLevel -= 1

  def newLine(): Unit = {
    writer.write(config.lineEnding)
    isStartOfLine = true
  }
  
  def writeSpace(): Unit = {
      writer.write(" ")
  }
  
  def writeComma(): Unit = {
      writer.write(",")
  }
  
  def ensureIndentation(): Unit = {
    if (isStartOfLine) {
      writer.write(" " * (indentationLevel * config.indentSize))
      isStartOfLine = false
    }
  }
  
  def flush(): Unit = writer.flush()
}
