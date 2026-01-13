package zio.schema.toon

import java.io.{BufferedReader, Reader}
import scala.collection.mutable
import scala.util.Try

/**
 * A bespoke parser for the TOON format.
 */
class ToonReader(reader: Reader, config: ToonReaderConfig) {
  private val br = new BufferedReader(reader)
  private var currentLine: String = _
  private var currentIndent: Int = -1
  private var isEof = false
  
  private var injectedLine: Option[(String, Int)] = None

  advance()

  private def advance(): Unit = {
    if (injectedLine.isDefined) {
        val (line, indent) = injectedLine.get
        currentLine = line
        currentIndent = indent
        injectedLine = None
        return
    }
      
    var line = br.readLine()
    if (line == null) {
      isEof = true
      currentLine = null
      currentIndent = -1
    } else {
      val trimmed = line.trim
      if (trimmed.isEmpty) {
        advance() 
      } else {
        val indent = line.takeWhile(_ == ' ').length
        currentLine = trimmed
        currentIndent = indent
      }
    }
  }
  
  def injectLine(line: String, indent: Int): Unit = {
      injectedLine = Some((line, indent))
      val oldLine = currentLine
      val oldIndent = currentIndent
      currentLine = line
      currentIndent = indent
  }

  def peekIndent(): Int = if (isEof) -1 else currentIndent
  def peekLine(): String = if (isEof) null else currentLine

  def consumeLine(): String = {
    val line = currentLine
    advance()
    line
  }

  // --- Value Parsing ---

  def readString(): String = {
    val line = consumeLine()
    stripQuotes(line)
  }
  
  private def stripQuotes(line: String): String = {
    if (line.startsWith("\"") && line.endsWith("\"")) {
      line.substring(1, line.length - 1)
        .replace("\\\"", "\"")
        .replace("\\n", "\n")
        .replace("\\r", "\r")
        .replace("\\t", "\t")
        .replace("\\\\", "\\")
    } else {
      line
    }
  }

  // --- Structure Parsing ---

  def readField(baseIndent: Int): (String, Option[String]) = {
    val line = currentLine
    val colonIdx = line.indexOf(':')
    
    if (colonIdx == -1) throw new RuntimeException(s"Expected key:value pair, got: $line")

    val key = line.substring(0, colonIdx).trim
    val valuePart = line.substring(colonIdx + 1).trim

    consumeLine() 

    if (valuePart.isEmpty) (key, None) else (key, Some(valuePart))
  }
  
  // --- Array Parsing ---
  
  def readInlineList(): Seq[String] = {
      val line = consumeLine()
      val result = mutable.ListBuffer[String]()
      var start = 0
      var inQuote = false
      var i = 0
      
      while (i < line.length) {
          val c = line.charAt(i)
          if (c == '"') {
             // Handle escaped quotes logic simplified: just toggle
             // For strict correctness we need to look behind for backslash
             if (i == 0 || line.charAt(i - 1) != '\\') {
                 inQuote = !inQuote
             }
          } else if (c == ',' && !inQuote) {
              result += line.substring(start, i).trim
              start = i + 1
          }
          i += 1
      }
      // Add last segment
      if (start < line.length) {
          result += line.substring(start).trim
      } else if (line.endsWith(",")) {
          // Empty last element logic? Usually empty string.
          // result += "" // Omit for now unless strict spec requires it
      }
      
      result.toSeq
  }

  def fail(msg: String): Nothing = throw new RuntimeException(msg)
}
