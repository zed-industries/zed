===============================================================================
=     B i e n v e n i d o   a l   t u t o r   d e   V I M  -  Versión 1.7     =
===============================================================================

     Vim es un editor muy potente que dispone de muchos comandos, demasiados
     para ser explicados en un tutor como éste. Este tutor está diseñado
     para describir suficientes comandos para que usted sea capaz de
     aprender fácilmente a usar Vim como un editor de propósito general.

     El tiempo necesario para completar el tutor es aproximadamente de 30
     minutos, dependiendo de cuánto tiempo se dedique a la experimentación.

     Los comandos de estas lecciones modificarán el texto. Haga una copia de
     este fichero para practicar (con «vimtutor» esto ya es una copia).

     Es importante recordar que este tutor está pensado para enseñar con
     la práctica. Esto significa que es necesario ejecutar los comandos
     para aprenderlos adecuadamente. Si únicamente lee el texto, ¡se le
     olvidarán los comandos.

     Ahora, asegúrese de que la tecla de bloqueo de mayúsculas NO está
     activada y pulse la tecla	j  lo suficiente para mover el cursor
     de forma que la Lección 1.1 ocupe completamente la pantalla.
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		     Lección 1.1: MOVER EL CURSOR

 ** Para mover el cursor, pulse las teclas h,j,k,l de la forma indicada. **
      ^
      k       Indicación: La tecla h está a la izquierda y lo mueve a la izquierda.
 < h	 l >		  La tecla l está a la derecha y lo mueve a la derecha.
      j			  La tecla j parece una flecha que apunta hacia abajo.
      v

  1. Mueva el cursor por la pantalla hasta que se sienta cómodo con ello.

  2. Mantenga pulsada la tecla (j) hasta que se repita «automágicamente».
     Ahora ya sabe como llegar a la lección siguiente.

  3. Utilizando la tecla abajo, vaya a la lección 1.2.

NOTA: Si alguna vez no está seguro sobre algo que ha tecleado, pulse <ESC>
      para situarse en modo Normal. Luego vuelva a teclear la orden que deseaba.

NOTA: Las teclas de movimiento del cursor también funcionan. Pero usando
      hjkl podrá moverse mucho más rápido una vez que se acostumbre a ello.
      ¡De verdad!

~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		    Lección 1.2: SALIR DE VIM

  ¡¡ NOTA: Antes de ejecutar alguno de los siguientes pasos lea primero
	   la lección entera!!

  1. Pulse la tecla <ESC> (para asegurarse de que está en modo Normal).

  2. Escriba:  :q! <INTRO>
     Esto provoca la salida del editor DESCARTANDO cualquier cambio que haya hecho.

  3. Regrese aquí ejecutando el comando que le trajo a este tutor.
     Éste puede haber sido:   vimtutor <INTRO>

  4. Si ha memorizado estos pasos y se siente con confianza, ejecute los
     pasos 1 a 3 para salir y volver a entrar al editor. 

NOTA:  :q! <INTRO> descarta cualquier cambio que haya realizado.
       En próximas lecciones aprenderá cómo guardar los cambios en un archivo.

  5. Mueva el cursor hasta la Lección 1.3.


~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		   Lección 1.3: EDITAR TEXTO - BORRAR

  ** Pulse  x  para eliminar el carácter bajo el cursor. **

  1. Mueva el cursor a la línea de abajo señalada con --->.

  2. Para corregir los errores, mueva el cursor hasta que esté sobre el
     carácter que va a ser borrado.

  3. Pulse la tecla  x	para eliminar el carácter no deseado.

  4. Repita los pasos 2 a 4 hasta que la frase sea la correcta.

---> La vvaca saltóó soobree laa luuuuna.

  5. Ahora que la línea esta correcta, continúe con la Lección 1.4.

NOTA: A medida que vaya avanzando en este tutor no intente memorizar,
      aprenda practicando.


~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		   Lección 1.4: EDITAR TEXTO - INSERTAR

         ** Pulse  i  para insertar texto. **

  1. Mueva el cursor a la primera línea de abajo señalada con --->.

  2. Para hacer que la primera línea sea igual que la segunda, mueva el
     cursor hasta que esté sobre el carácter ANTES del cual el texto va a ser
     insertado.

  3. Pulse  i  y escriba los caracteres a añadir.

  4. A medida que sea corregido cada error pulse <ESC> para volver al modo
     Normal. Repita los pasos 2 a 4 para corregir la frase.

---> Flta texto en esta .
---> Falta algo de texto en esta línea.

  5. Cuando se sienta cómodo insertando texto pase vaya a la lección 1.5.


~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		     Lección 1.5: EDITAR TEXTO - AÑADIR


			** Pulse  A  para añadir texto. **

  1. Mueva el cursor a la primera línea inferior marcada con --->.
     No importa sobre qué carácter está el cursor en esta línea.

  2. Pulse  A  y escriba el texto necesario.

  3. Cuando el texto haya sido añadido pulse <ESC> para volver al modo Normal.

  4. Mueva el cursor a la segunda línea marcada con ---> y repita los
     pasos 2 y 3 para corregir esta frase.

---> Falta algún texto en es
     Falta algún texto en esta línea.
---> También falta alg
     También falta algún texto aquí.

  5. Cuando se sienta cómodo añadiendo texto pase a la lección 1.6.

~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		     Lección 1.6: EDITAR UN ARCHIVO

		    ** Use  :wq  para guardar un archivo y salir **

 !! NOTA: Antes de ejecutar los siguientes pasos, lea la lección entera!!

  1.  Si tiene acceso a otra terminal, haga lo siguiente en ella.
      Si no es así, salga de este tutor como hizo en la lección 1.2:  :q!

  2. En el símbolo del sistema escriba este comando:  vim archivo.txt <INTRO>
     'vim' es el comando para arrancar el editor Vim, 'archivo.txt'
     es el nombre del archivo que quiere editar
     Utilice el nombre de un archivo que pueda cambiar.

  3. Inserte y elimine texto como ya aprendió en las lecciones anteriores.

  4. Guarde el archivo con los cambios y salga de Vim con:  :wq <INTRO>

  5. Si ha salido de vimtutor en el paso 1 reinicie vimtutor y baje hasta
     el siguiente sumario.

  6. Después de leer los pasos anteriores y haberlos entendido: hágalos.

~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			    RESUMEN DE LA LECCIÓN 1


  1. El cursor se mueve utilizando las teclas de las flechas o las teclas hjkl.
	 h (izquierda)	   j (abajo)	  k (arriba)	  l (derecha)

  2. Para acceder a Vim desde el símbolo del sistema escriba:
     vim NOMBREARCHIVO <INTRO>

  3. Para salir de Vim escriba: <ESC> :q! <INTRO> para eliminar todos
     los cambios.
     O escriba:  <ESC>  :wq  <INTRO> para guardar los cambios.

  4. Para borrar un carácter bajo el cursor en modo Normal pulse:  x

  5. Para insertar o añadir texto escriba:
     i  escriba el texto a insertar <ESC> inserta el texto antes del cursor
	 A  escriba el texto a añadir <ESC> añade texto al final de la línea

NOTA: Pulsando <ESC> se vuelve al modo Normal o cancela una orden no deseada
      o incompleta.

Ahora continúe con la Lección 2.

~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		     Lección 2.1:  COMANDOS PARA BORRAR


          ** Escriba dw para borrar una palabra **


  1. Pulse <ESC> para asegurarse de que está en el modo Normal.

  2. Mueva el cursor a la línea inferior señalada con --->.

  3. Mueva el cursor al comienzo de una palabra que desee borrar.

  4. Pulse   dw   para hacer que la palabra desaparezca.

  NOTA: La letra  d  aparecerá en la última línea inferior derecha 
    de la pantalla mientras la escribe. Vim está esperando que escriba  w .
    Si ve otro carácter que no sea  d  escribió algo mal, pulse <ESC> y
    comience de nuevo.

---> Hay algunas palabras pásalo bien que no pertenecen papel a esta frase.

  5. Repita los pasos 3 y 4 hasta que la frase sea correcta y pase a la
     lección 2.2.


~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		    Lección 2.2: MÁS COMANDOS PARA BORRAR


	  ** Escriba  d$  para borrar hasta el final de la línea. **

  1. Pulse  <ESC>  para asegurarse de que está en el modo Normal.

  2. Mueva el cursor a la línea inferior señalada con --->.

  3. Mueva el cursor al final de la línea correcta (DESPUÉS del primer . ).

  4. Escriba  d$  para borrar hasta el final de la línea.

---> Alguien ha escrito el final de esta línea dos veces. esta línea dos veces.

  5. Pase a la lección 2.3 para entender qué está pasando.



~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		    Lección 2.3: SOBRE OPERADORES Y MOVIMIENTOS


  Muchos comandos que cambian texto están compuestos por un operador y un
  movimiento.
  El formato para eliminar un comando con el operador de borrado  d  es el
  siguiente:

    d   movimiento

  Donde:
    d          - es el operador para borrar.
    movimiento - es sobre lo que el comando va a operar (lista inferior).

  Una lista resumida de movimientos:
   w - hasta el comienzo de la siguiente palabra, EXCLUYENDO su primer
       carácter.
   e - hasta el final de la palabra actual, INCLUYENDO el último carácter.
   $ - hasta el final de la línea, INCLUYENDO el último carácter.

 Por tanto, al escribir  de  borrará desde la posición del cursor, hasta
 el final de la palabra.

NOTA: Pulsando únicamente el movimiento estando en el modo Normal sin un
      operador, moverá el cursor como se especifica en la lista anterior.

~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		  Lección 2.4: UTILIZAR UN CONTADOR PARA UN MOVIMIENTO


   ** Al escribir un número antes de un movimiento, lo repite esas veces. **

  1. Mueva el cursor al comienzo de la línea marcada con --->.

  2. Escriba  2w  para mover el cursor dos palabras hacia adelante.

  3. Escriba  3e  para mover el cursor al final de la tercera palabra hacia
     adelante.

  4. Escriba  0  (cero) para colocar el cursor al inicio de la línea.

  5. Repita el paso 2 y 3 con diferentes números.

---> Esto es solo una línea con palabras donde poder moverse.

  6. Pase a la lección 2.5.




~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		     Lección 2.5: UTILIZAR UN CONTADOR PARA BORRAR MAS


   ** Al escribir un número con un operador lo repite esas veces. **

  En combinación con el operador de borrado y el movimiento mencionado
  anteriormente, añada un contador antes del movimiento para eliminar más:
	 d   número   movimiento

  1. Mueva el cursor al inicio de la primera palabra en MAYÚSCULAS en la
     línea marcada con --->.

  2. Escriba  d2w  para eliminar las dos palabras en MAYÚSCULAS.

  3. Repita los pasos 1 y 2 con diferentes contadores para eliminar
     las siguientes palabras en MAYÚSCULAS con un comando.

--->  Esta ABC DE serie FGHI JK LMN OP de palabras ha sido Q RS TUV limpiada.





~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			 Lección 2.6: OPERACIÓN EN LÍNEAS


		   ** Escriba  dd   para eliminar una línea completa. **

  Debido a la frecuencia con que se elimina una línea completa, los
  diseñadores de Vi, decidieron que sería más sencillo simplemente escribir
  dos letras d para eliminar una línea.

  1. Mueva el cursor a la segunda línea del párrafo inferior.
  2. Escriba  dd  para eliminar la línea.
  3. Ahora muévase a la cuarta línea.
  4. Escriba   2dd   para eliminar dos líneas a la vez.

--->  1)  Las rosas son rojas,
--->  2)  El barro es divertido,
--->  3)  La violeta es azul,
--->  4)  Tengo un coche,
--->  5)  Los relojes dan la hora,
--->  6)  El azúcar es dulce
--->  7)  Y también lo eres tú.

La duplicación para borrar líneas también funcionan con los operadores
mencionados anteriormente.

~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		     Lección 2.7: EL MANDATO DESHACER


   ** Pulse  u	para deshacer los últimos comandos,
	     U	para deshacer una línea entera.       **

  1. Mueva el cursor a la línea inferior señalada con ---> y sitúelo bajo el
     primer error.
  2. Pulse  x  para borrar el primer carácter no deseado.
  3. Pulse ahora  u  para deshacer el último comando ejecutado.
  4. Ahora corrija todos los errores de la línea usando el comando  x.
  5. Pulse ahora  U  mayúscula para devolver la línea a su estado original.
  6. Pulse ahora  u  unas pocas veces para deshacer lo hecho por  U  y los
     comandos previos.
  7. Ahora pulse CTRL-R (mantenga pulsada la tecla CTRL y pulse R) unas
     cuantas veces para volver a ejecutar los comandos (deshacer lo deshecho).

---> Corrrija los errores dee esttta línea y vuuelva a ponerlos coon deshacer.

  8. Estos son unos comandos muy útiles. Ahora vayamos al resumen de la
     lección 2.




~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			    RESUMEN DE LA LECCIÓN 2

  1. Para borrar desde el cursor hasta siguiente palabra pulse:	     dw
  2. Para borrar desde el cursor hasta el final de la palabra pulse: de
  3. Para borrar desde el cursor hasta el final de una línea pulse:	 d$
  4. Para borrar una línea entera pulse:                             dd

  5. Para repetir un movimiento anteponga un número:  2w
  6. El formato para un comando de cambio es:
               operador  [número]  movimiento
     donde:
       comando    - es lo que hay que hacer, por ejemplo,  d  para borrar
       [número]   - es un número opcional para repetir el movimiento
       movimiento - se mueve sobre el texto sobre el que operar, como
		            w (palabra), $ (hasta el final de la línea), etc.
  7. Para moverse al inicio de la línea utilice un cero:  0

  8. Para deshacer acciones previas pulse:		         u (u minúscula)
     Para deshacer todos los cambios de una línea pulse: U (U mayúscula)
     Para deshacer lo deshecho pulse:			         CTRL-R


~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			 Lección 3.1: EL COMANDO «PUT» (poner)

** Pulse  p  para poner (pegar) después del cursor lo último que ha borrado. **

  1. Mueva el cursor a la primera línea inferior marcada con --->.

  2. Escriba  dd  para borrar la línea y almacenarla en un registro de Vim.

  3. Mueva el cursor a la línea c) por ENCIMA de donde debería estar 
     la línea eliminada.

  4. Pulse   p	para pegar la línea borrada por debajo del cursor.

  5. Repita los pasos 2 a 4 para poner todas las líneas en el orden correcto.

---> d) ¿Puedes aprenderla tú?
---> b) La violeta es azul,
---> c) La inteligencia se aprende,
---> a) Las rosas son rojas,
     

~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		       Lección 3.2: EL COMANDO REEMPLAZAR


  ** Pulse  rx  para reemplazar el carácter bajo el cursor con  x . **

  1. Mueva el cursor a la primera línea inferior marcada con --->.

  2. Mueva el cursor para situarlo sobre el primer error.

  3. Pulse   r	 y después el carácter que debería ir ahí.

  4. Repita los pasos 2 y 3 hasta que la primera sea igual a la segunda.

---> ¡Cuendo esta línea fue rscrita alguien pulso algunas teclas equibocadas!
---> ¡Cuando esta línea fue escrita alguien pulsó algunas teclas equivocadas!

  5. Ahora pase a la lección 3.3.

NOTA: Recuerde que debería aprender practicando.



~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			Lección 3.3: EL COMANDO CAMBIAR


     ** Para cambiar hasta el final de una palabra, escriba  ce . **

  1. Mueva el cursor a la primera línea inferior marcada con --->.

  2. Sitúe el cursor en la u de lubrs.

  3. Escriba  ce  y corrija la palabra (en este caso, escriba 'ínea').

  4. Pulse <ESC> y mueva el cursor al siguiente error que debe ser cambiado.

  5. Repita los pasos 3 y 4 hasta que la primera frase sea igual a la segunda.

---> Esta lubrs tiene unas pocas pskavtad que corregir usem el comando change.
---> Esta línea tiene unas pocas palabras que corregir usando el comando change.

Tenga en cuenta que  ce  elimina la palabra y entra en el modo Insertar.
                    cc  hace lo mismo para toda la línea.


~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		      Lección 3.4: MÁS CAMBIOS USANDO c

   ** El operador change se utiliza con los mismos movimientos que delete. **

  1. El operador change funciona de la misma forma que delete. El formato es:

       c   [número]   movimiento

  2. Los movimientos son también los mismos, tales como  w (palabra) o 
  $ (fin de la línea).

  3. Mueva el cursor a la primera línea inferior señalada con --->.

  4. Mueva el cursor al primer error.

  5. Pulse  c$  y escriba el resto de la línea para que sea como la segunda
     y pulse <ESC>.

---> El final de esta línea necesita alguna ayuda para que sea como la segunda.
---> El final de esta línea necesita ser corregido usando el comando  c$.

NOTA: Puede utilizar el retorno de carro para corregir errores mientras escribe.

~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			    RESUMEN DE LA LECCIÓN 3


  1. Para volver a poner o pegar el texto que acaba de ser borrado,
     escriba  p . Esto pega el texto después del cursor (si se borró una
     línea, al pegarla, esta se situará en la línea debajo del cursor).

  2. Para reemplazar el carácter bajo el cursor, pulse	r   y luego el
     carácter que quiere que esté en ese lugar.

  3. El operador change le permite cambiar desde la posición del cursor
     hasta donde el movimiento indicado le lleve. Por ejemplo, pulse  ce
     para cambiar desde el cursor hasta el final de la palabra, o  c$
     para cambiar hasta el final de la línea.

  4. El formato para change es:

	 c   [número]   movimiento

  Pase ahora a la lección siguiente.


~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
	       Lección 4.1: UBICACIÓN DEL CURSOR Y ESTADO DEL ARCHIVO

 ** Pulse CTRL-G para mostrar su situación en el fichero y su estado.
    Pulse G para moverse a una determinada línea del fichero. **

NOTA: ¡¡Lea esta lección entera antes de ejecutar cualquiera de los pasos!!

  1. Mantenga pulsada la tecla Ctrl y pulse  g . Le llamamos a esto CTRL-G.
     Aparecerá un mensaje en la parte inferior de la página con el nombre
     del archivo y la posición en este. Recuerde el número de línea
     para el paso 3.

NOTA: Quizás pueda ver la posición del cursor en la esquina inferior derecha
      de la pantalla. Esto ocurre cuando la opción 'ruler' (regla) está
      habilitada (consulte  :help 'ruler'  )

  2. Pulse  G  para mover el cursor hasta la parte inferior del archivo.
     Pulse  gg  para mover el cursor al inicio del archivo.

  3. Escriba el número de la línea en la que estaba y después  G  . Esto
     le volverá a la línea en la que estaba cuando pulsó CTRL-G.

  4. Si se siente seguro en poder hacer esto ejecute los pasos 1 a 3.

~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			Lección 4.2: EL COMANDO «SEARCH» (buscar)

     ** Escriba  /  seguido de una frase para buscar la frase. **

  1. En modo Normal pulse el carácter  / . Fíjese que tanto el carácter  /
     como el cursor aparecen en la última línea de la pantalla, lo mismo
     que el comando  : .

  2. Escriba ahora   errroor   <INTRO>. Esta es la palabra que quiere buscar.

  3. Para repetir la búsqueda de la misma frase otra vez, simplemente pulse  n .
     Para buscar la misma frase en la dirección opuesta, pulse  N .

  4. Si quiere buscar una frase en la dirección opuesta (hacia arriba),
     utilice el comando  ?  en lugar de  / .
  
  5. Para regresar al lugar de donde procedía pulse  CTRL-O  (Mantenga pulsado
  Ctrl mientras pulsa la letra o). Repita el proceso para regresar más atrás.
  CTRL-I va hacia adelante.

---> "errroor" no es la forma correcta de escribir error, errroor es un error.

NOTA: Cuando la búsqueda llega al final del archivo, continuará desde el
      comienzo, a menos que la opción 'wrapscan' haya sido desactivada.

~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
	       Lección 4.3: BÚSQUEDA PARA COMPROBAR PARÉNTESIS

   ** Pulse %  para encontrar el paréntesis correspondiente a ),] o } . **

  1. Sitúe el cursor en cualquiera de los caracteres (, [ o { en la línea 
     inferior señalada con --->.

  2. Pulse ahora el carácter  %  .

  3. El cursor se moverá a la pareja de cierre del paréntesis, corchete
     o llave correspondiente.

  4. Pulse  %  para mover el cursor a la otra pareja del carácter.

  5. Mueva el cursor a otro (,),[,],{ o } y vea lo que hace % .

---> Esto ( es una línea de prueba con (, [, ], {, y } en ella. ))

NOTA: ¡Esto es muy útil en la detección de errores en un programa con
      paréntesis, corchetes o llaves sin pareja.
      


~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		  Lección 4.4: EL COMANDO SUSTITUIR


    ** Escriba	:s/viejo/nuevo/g para sustituir 'viejo' por 'nuevo'. **

  1. Mueva el cursor a la línea inferior señalada con --->.

  2. Escriba  :s/laas/las/  <INTRO> . Tenga en cuenta que este mandato cambia
     sólo la primera aparición en la línea de la expresión a cambiar.
  
  3. Ahora escriba :/laas/la/g . Al añadir la opción  g  esto significa
     que hará la sustitución global en la línea, cambiando todas las
     ocurrencias del término "laas" en la línea.

---> Laas mejores épocas para ver laas flores son laas primaveras.

  4. Para cambiar cada ocurrencia de la cadena de caracteres entre dos líneas,
   Escriba  :#,#s/viejo/nuevo/g  donde #,# son los números de línea del rango
                                 de líneas donde se realizará la sustitución.
   Escriba  :%s/old/new/g        para cambiar cada ocurrencia en todo el
                                 archivo.
   Escriba  :%s/old/new/gc       para encontrar cada ocurrencia en todo el 
                                 archivo, pidiendo confirmación para 
                                 realizar la sustitución o no.

~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			    RESUMEN DE LA LECCIÓN 4


  1. CTRL-G  muestra la posición del cursor en el fichero y su estado.
             G  mueve el cursor al final del archivo.
     número  G  mueve el cursor a ese número de línea.
            gg  mueve el cursor a la primera línea del archivo.

  2. Escribiendo  /  seguido de una frase busca la frase hacia ADELANTE.
     Escribiendo  ?  seguido de una frase busca la frase hacia ATRÁS.
     Después de una búsqueda pulse  n  para encontrar la aparición
     siguiente en la misma dirección o  N  para buscar en dirección opuesta.

  3. Pulsando  %  cuando el cursor esta sobre (,), [,], { o } localiza
     la pareja correspondiente.

  4. Para cambiar viejo en el primer nuevo en una línea escriba  :s/viejo/nuevo
   Para cambiar todos los viejo por nuevo en una línea escriba :s/viejo/nuevo/g
   Para cambiar frases entre dos números de líneas escriba  :#,#s/viejo/nuevo/g
   Para cambiar viejo por nuevo en todo el fichero escriba  :%s/viejo/nuevo/g
   Para pedir confirmación en cada caso añada  'c'	    :%s/viejo/nuevo/gc


~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		Lección 5.1: CÓMO EJECUTAR UN MANDATO EXTERNO


  ** Escriba  :!  seguido de un comando externo para ejecutar ese comando. **

  1. Escriba el conocido comando  :  para situar el cursor al final de la
     pantalla. Esto le permitirá introducir un comando.

  2. Ahora escriba el carácter ! (signo de admiración). Esto le permitirá
     ejecutar cualquier mandato del sistema.

  3. Como ejemplo escriba   ls	 después del ! y luego pulse <INTRO>. Esto
     le mostrará una lista de su directorio, igual que si estuviera en el
     símbolo del sistema. Si  ls  no funciona utilice	:!dir	.

NOTA: De esta manera es posible ejecutar cualquier comando externo,
      también incluyendo argumentos.

NOTA: Todos los comando   :   deben finalizarse pulsando <INTRO>.
      De ahora en adelante no siempre se mencionará.


~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		     Lección 5.2: MÁS SOBRE GUARDAR FICHEROS


     ** Para guardar los cambios hechos en un fichero,
	escriba  :w NOMBRE_DE_FICHERO **

  1. Escriba  :!dir  o	:!ls  para ver una lista de los archivos 
     de su directorio.
     Ya sabe que debe pulsar <INTRO> después de ello.

  2. Elija un nombre de fichero que todavía no exista, como TEST.

  3. Ahora escriba   :w TEST  (donde TEST es el nombre de fichero elegido).

  4. Esta acción guarda todo el fichero  (Vim Tutor)  bajo el nombre TEST.
     Para comprobarlo escriba	:!dir  o  :!ls  de nuevo y vea su directorio.

NOTA: Si saliera de Vim y volviera a entrar de nuevo con  vim TEST  , el
      archivo sería una copia exacta del tutorial cuando lo guardó.

  5. Ahora elimine el archivo escribiendo (Windows):  :!del TEST
                                        o (Unix):     :!rm TEST


~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
	       Lección 5.3: SELECCIONAR TEXTO PARA GUARDAR


   ** Para guardar parte del archivo, escriba  v  movimiento  :w ARCHIVO **

  1. Mueva el cursor a esta línea.

  2. Pulse  v  y mueva el cursor hasta el quinto elemento inferior. Vea que
     el texto es resaltado.

  3. Pulse el carácter  :  en la parte inferior de la pantalla aparecerá
     :'<,'>

  4. Pulse  w TEST  , donde TEST es un nombre de archivo que aún no existe.
     Verifique que ve  :'<,'>w TEST  antes de pulsar <INTRO>.

  5. Vim escribirá las líneas seleccionadas en el archivo TEST. Utilice
     :!dir  o  :!ls  para verlo. ¡No lo elimine todavía! Lo utilizaremos
     en la siguiente lección.

NOTA: Al pulsar  v  inicia la selección visual. Puede mover el cursor para
      hacer la selección más grande o pequeña. Después puede utilizar un
      operador para hacer algo con el texto. Por ejemplo,  d  eliminará
      el texto seleccionado.


~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		Lección 5.4: RECUPERANDO Y MEZCLANDO FICHEROS


 ** Para insertar el contenido de un fichero escriba :r NOMBRE_DEL_FICHERO **

  1. Sitúe el cursor justo por encima de esta línea.

NOTA: Después de ejecutar el paso 2 verá texto de la lección 5.3. Después
      DESCIENDA hasta ver de nuevo esta lección.

  2. Ahora recupere el archivo TEST utilizando el comando  :r TEST  donde
     TEST es el nombre que ha utilizado.
     El archivo que ha recuperado se colocará debajo de la línea donde
     se encuentra el cursor.

  3. Para verificar que se ha recuperado el archivo, suba el cursor y 
     compruebe que ahora hay dos copias de la lección 5.3, la original y
     la versión del archivo.

NOTA: También puede leer la salida de un comando externo. Por ejemplo,
      :r !ls  lee la salida del comando ls y lo pega debajo de la línea
      donde se encuentra el cursor.


~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			   RESUMEN DE LA LECCIÓN 5


  1.  :!comando  ejecuta un comando externo.

      Algunos ejemplos útiles son:
      (Windows)     (Unix)
	  :!dir          :!ls           - muestra el contenido de un directorio.
	  :!del ARCHIVO  :!rm ARCHIVO   -  borra el fichero ARCHIVO.

  2.  :w ARCHIVO escribe el archivo actual de Vim en el disco con el 
      nombre de ARCHIVO.

  3.  v  movimiento  :w ARCHIVO  guarda las líneas seleccionadas visualmente
      en el archivo ARCHIVO.

  4.  :r ARCHIVO  recupera del disco el archivo ARCHIVO y lo pega debajo
      de la posición del cursor.

  5.  :r !dir  lee la salida del comando dir y lo pega debajo de la
      posición del cursor.


~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			 Lección 6.1: EL COMANDO OPEN


	 ** Pulse  o  para abrir una línea debajo del cursor
	    y situarle en modo Insertar **

  1. Mueva el cursor a la línea inferior señalada con --->.

  2. Pulse la letra minúscula  o  para abrir una línea por DEBAJO del cursor
     y situarle en modo Insertar.
  
  3. Ahora escriba algún texto y después pulse <ESC> para salir del modo
     insertar.

---> Después de pulsar  o  el cursor se sitúa en la línea abierta en modo Insertar.

  4. Para abrir una línea por ENCIMA del cursor, simplemente pulse una O
     mayúscula, en lugar de una o minúscula. Pruebe esto en la línea siguiente.

---> Abra una línea sobre esta pulsando O cuando el cursor está en esta línea.



~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			Lección 6.2: EL COMANDO APPEND (añadir)


	 ** Pulse  a  para insertar texto DESPUÉS del cursor. **

  1. Mueva el cursor al inicio de la primera línea inferior señalada con --->.

  2. Escriba  e  hasta que el cursor esté al final de  lín .

  3. Escriba una  a  (minúscula) para añadir texto DESPUÉS del cursor.

  4. Complete la palabra como en la línea inferior. Pulse <ESC> para salir
     del modo insertar.
  
  5. Utilice  e  para moverse hasta la siguiente palabra incompleta y 
     repita los pasos 3 y 4.

---> Esta lín le permit prati cómo añad texto a una línea.
---> Esta línea le permitirá practicar cómo añadir texto a una línea.

NOTA: a, i y A todos entran en el modo Insertar, la única diferencia es
      dónde ubican los caracteres insertados.

~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		     Lección 6.3: OTRA VERSIÓN DE REPLACE (remplazar)


    ** Pulse una  R  mayúscula para sustituir más de un carácter. **

  1. Mueva el cursor a la primera línea inferior señalada con --->. Mueva
     el cursor al inicio de la primera  xxx .

  2. Ahora pulse  R  y escriba el número que aparece en la línea inferior,
     esto reemplazará el texto xxx .
  
  3. Pulse <ESC> para abandonar el modo Reemplazar. Observe que el resto de
     la línea permanece sin modificaciones.

  4. Repita los pasos para reemplazar el texto xxx que queda.

---> Sumar 123 a xxx da un resultado de xxx.
---> Sumar 123 a 456 da un resultado de 579.

NOTA: El modo Reemplazar es como el modo Insertar, pero cada carácter escrito
      elimina un carácter ya existente.

~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			 Lección 6.4: COPIAR Y PEGAR TEXTO



	  ** Utilice el operador  y  para copiar texto y  p  para pegarlo. **

  1. Mueva el cursor a la línea inferior marcada con ---> y posicione el 
     cursor después de "a)". 

  2. Inicie el modo Visual con  v  y mueva el cursor justo antes de "primer".

  3. Pulse  y  para copiar ("yank") el texto resaltado.

  4. Mueva el cursor al final de la siguiente línea mediante:  j$

  5. Pulse  p  para poner (pegar) el texto. Después escriba: el segundo <ESC>.

  6. Utilice el modo visual para seleccionar " elemento.", y cópielo con  y
     mueva el cursor al final de la siguiente línea con j$  y pegue el texto
     recién copiado con  p .

--->  a) este es el primer elemento.
      b)

NOTA: También puede utilizar  y  como un operador:  yw  copia una palabra,
      yy  copia la línea completa donde está el cursor, después  p  pegará
      esa línea.
     
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			    Lección 6.5: ACTIVAR (SET) UNA OPCIÓN


	  ** Active una opción para buscar o sustituir ignorando si está
         en mayúsculas o minúsculas el texto. **

  1. Busque la cadena de texto 'ignorar' escribiendo:  /ignorar <INTRO>
     Repita la búsqueda varias veces pulsando  n .

  2. Active la opción 'ic' (Ignore case o ignorar mayúsculas y minúsculas) 
     mediante:   :set ic

  3. Ahora busque de nuevo 'ignorar' pulsando  n
     Observe que ahora también se encuentran Ignorar e IGNORAR.

  4. Active las opciones 'hlsearch' y 'incsearch' escribiendo:  :set hls is

  5. Ahora escriba de nuevo el comando de búsqueda y vea qué ocurre:  /ignore <INTRO>

  6. Para inhabilitar el ignorar la distinción de mayúsculas y minúsculas     
     escriba:  :set noic

NOTA:  Para eliminar el resaltado de las coincidencias escriba:   :nohlsearch
NOTA:  Si quiere ignorar las mayúsculas y minúsculas, solo para un comando
       de búsqueda, utilice  \c  en la frase:  /ignorar\c <INTRO>
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			   RESUMEN DE LA LECCIÓN 6


  1. Escriba  o  para abrir una línea por DEBAJO de la posición del cursor y 
     entrar en modo Insertar.
     Escriba  O  para abrir una línea por ENCIMA de la posición del cursor y
     entrar en modo Insertar

  2. Escriba  a  para insertar texto DESPUÉS del cursor.
     Escriba  A  para insertar texto al final de la línea.

  3. El comando  e  mueve el cursor al final de una palabra.

  4. El operador  y  copia (yank) texto,  p  lo pega (pone).

  5. Al escribir una  R  mayúscula entra en el modo Reemplazar hasta que
     se pulsa  <ESC>  .

  6. Al escribir ":set xxx" activa la opción "xxx".  Algunas opciones son:
  	'ic' 'ignorecase'	ignorar mayúsculas/minúsculas al buscar
	'is' 'incsearch'	mostrar las coincidencias parciales para la búsqueda
                        de una frase
	'hls' 'hlsearch'	resalta todas las coincidencias de la frases
     Puedes utilizar tanto los nombre largos o cortos de las opciones.

  7. Añada "no" para inhabilitar una opción:   :set noic

~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		  Lección 7: OBTENER AYUDA


		 ** Utilice el sistema de ayuda en línea **

  Vim dispone de un sistema de ayuda en línea. Para comenzar, pruebe una
  de estas tres formas:
	- pulse la tecla <AYUDA> (si dispone de ella)
	- pulse la tecla <F1> (si dispone de ella)
	- escriba   :help <INTRO>

  Lea el texto en la ventana de ayuda para descubrir cómo funciona la ayuda.
  Escriba  CTRL-W CTRL-W  para saltar de una ventana a otra.
  Escriba    :q <INTRO>   para cerrar la ventana de ayuda.

  Puede encontrar ayuda en casi cualquier tema añadiendo un argumento al
  comando «:help». Pruebe éstos (no olvide pulsar <INTRO>):

  :help w 
  :help c_CTRL-D
  :help insert-index 
  :help user-manual
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		      Lección 7.2: CREAR UN SCRIPT DE INICIO


			  ** Habilitar funcionalidades en Vim **

  Vim tiene muchas más funcionalidades que Vi, pero algunas están
  inhabilitadas de manera predeterminada.
  Para empezar a utilizar más funcionalidades debería crear un archivo
  llamado "vimrc".

  1. Comience a editar el archivo "vimrc". Esto depende de su sistema:
	:e ~/.vimrc		para Unix
	:e ~/_vimrc		para Windows

  2. Ahora lea el contenido del archivo "vimrc" de ejemplo:
	:r $VIMRUNTIME/vimrc_example.vim

  3. Guarde el archivo mediante:
	:w

  La próxima vez que inicie Vim, este usará el resaltado de sintaxis.
  Puede añadir todos sus ajustes preferidos a este archivo "vimrc".
  Para más información escriba  :help vimrc-intro

~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			     Lección 7.3: COMPLETADO


	      ** Completado de la línea de comandos con CTRL-D o <TAB> **

  1. Asegúrese de que Vim no está en el modo compatible:  :set nocp

  2. Vea qué archivos existen en el directorio con:  :!ls   o   :!dir

  3. Escriba el inicio de un comando:  :e

  4. Pulse  CTRL-D  y Vim mostrará una lista de comandos que empiezan con "e".

  5. Añada  d<TAB>  y Vim completará el nombre del comando a ":edit".

  6. Ahora añada un espacio y el inicio del nombre de un archivo:  :edit FIL

  7. Pulse <TAB>.  Vim completará el nombre (si solo hay uno).

NOTA:  El completado funciona con muchos comandos. Solo pulse CTRL-D o
       <TAB>.  Es especialmente útil para   :help .

~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			       RESUMEN DE LA LECCIÓN 7


  1. Escriba  :help  o pulse <F1> o <HELP>  para abrir la ventana de ayuda.

  2. Escriba  :help cmd  para encontrar ayuda sobre  cmd .

  3. Escriba  CTRL-W CTRL-W  para saltar a otra ventana.

  4. Escriba  :q  para cerrar la ventana de ayuda.

  5. Cree un fichero vimrc de inicio para guardar sus ajustes preferidos.

  6. Cuando escriba un comando  :  pulse CTRL-D para ver posibles opciones.
     Pulse <TAB> para utilizar una de las opciones de completado.







~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

  Aquí concluye el tutor de Vim. Está pensado para dar una visión breve del
  editor Vim, lo suficiente para permitirle usar el editor de forma bastante
  sencilla. Está muy lejos de estar completo pues Vim tiene muchísimos más
  comandos. Lea el siguiente manual de usuario: ":help user-manual".

  Para lecturas y estudios posteriores se recomienda el libro:
	Vim - Vi Improved - de Steve Oualline
	Editado por: New Riders
  El primer libro dedicado completamente a Vim. Especialmente útil para
  recién principiantes.
  Tiene muchos ejemplos e imágenes.
  Vea https://iccf-holland.org/click5.html

  Este tutorial ha sido escrito por Michael C. Pierce y Robert K. Ware,
  Colorado School of Mines utilizando ideas suministradas por Charles Smith,
  Colorado State University.
  E-mail: bware@mines.colorado.edu.

  Modificado para Vim por Bram Moolenaar.

~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    
  Traducido del inglés por:

  * Eduardo F. Amatria
    Correo electrónico: eferna1@platea.pntic.mec.es
  * Victorhck
    Correo electrónico: victorhck@opensuse.org

~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
