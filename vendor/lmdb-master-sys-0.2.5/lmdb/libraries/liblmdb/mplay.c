/* mplay.c - memory-mapped database log replay */
/*
 * Copyright 2011-2023 Howard Chu, Symas Corp.
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted only as authorized by the OpenLDAP
 * Public License.
 *
 * A copy of this license is available in the file LICENSE in the
 * top-level directory of the distribution or, alternatively, at
 * <http://www.OpenLDAP.org/license.html>.
 */
#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <time.h>
#include <string.h>
#include <ctype.h>
#include <assert.h>
#include <sys/types.h>
#include <sys/wait.h>

#include "lmdb.h"

#define E(expr) CHECK((rc = (expr)) == MDB_SUCCESS, #expr)
#define RES(err, expr) ((rc = expr) == (err) || (CHECK(!rc, #expr), 0))
#define CHECK(test, msg) ((test) ? (void)0 : ((void)fprintf(stderr, \
	"%s:%d: %s: %s\n", __FILE__, __LINE__, msg, mdb_strerror(rc)), abort()))

#define SCMP(s)	s, (sizeof(s)-1)
char inbuf[8192];
char *dbuf, *kbuf;
size_t dbufsize;
int maxkey;

#define SOFF(s)	(sizeof(s)+1)

#define MAXENVS	16
#define MAXTXNS	16
#define MAXCRSS	16

#define MAXPIDS	16

typedef struct crspair {
	void *tcrs;	/* scanned text pointer */
	MDB_cursor *rcrs;
} crspair;

typedef struct txnpair {
	void *ttxn;	/* scanned text pointer */
	MDB_txn *rtxn;
	crspair cursors[MAXCRSS];
	int ncursors;
} txnpair;

typedef struct envpair {
	void *tenv;
	MDB_env *renv;
	txnpair txns[MAXTXNS];
	int ntxns;
} envpair;

envpair envs[MAXENVS];
int nenvs;

envpair *lastenv;
txnpair *lasttxn;
crspair *lastcrs;

typedef struct pidpair {
	int tpid;
	pid_t rpid;
	int fdout, fdin;
} pidpair;

pidpair *lastpid;

pidpair pids[MAXPIDS];
int npids;

unsigned long lcount;

static int unhex(unsigned char *c2)
{
	int x, c;
	x = *c2++ & 0x4f;
	if (x & 0x40)
		x -= 55;
	c = x << 4;
	x = *c2 & 0x4f;
	if (x & 0x40)
		x -= 55;
	c |= x;
	return c;
}

int inhex(char *in, char *out)
{
	char *c2 = out;
	while (isxdigit(*in)) {
		*c2++ = unhex((unsigned char *)in);
		in += 2;
	}
	return c2 - out;
}

static void addenv(void *tenv, MDB_env *renv)
{
	assert(nenvs < MAXENVS);
	envs[nenvs].tenv = tenv;
	envs[nenvs].renv = renv;
	envs[nenvs].ntxns = 0;
	lastenv = envs+nenvs;
	nenvs++;
}

static envpair *findenv(void *tenv)
{
	int i;
	if (!lastenv || lastenv->tenv != tenv) {
		for (i=0; i<nenvs; i++)
			if (envs[i].tenv == tenv)
				break;
		assert(i < nenvs);
		lastenv = &envs[i];
	}
	return lastenv;
}

static void delenv(envpair *ep)
{
	int i = ep - envs;
	for (; i<nenvs-1; i++)
		envs[i] = envs[i+1];
	nenvs--;
	lastenv = NULL;
}

static void addtxn(void *tenv, void *ttxn, MDB_txn *rtxn)
{
	envpair *ep;
	txnpair *tp;

	ep = findenv(tenv);
	assert(ep->ntxns < MAXTXNS);
	tp = ep->txns+ep->ntxns;
	tp->ttxn = ttxn;
	tp->rtxn = rtxn;
	tp->ncursors = 0;
	ep->ntxns++;
	lasttxn = tp;
}

static txnpair *findtxn(void *ttxn)
{
	int i, j;
	if (lasttxn && lasttxn->ttxn == ttxn)
		return lasttxn;
	if (lastenv) {
		for (i=0; i<lastenv->ntxns; i++) {
			if (lastenv->txns[i].ttxn == ttxn) {
				lasttxn = lastenv->txns+i;
				return lasttxn;
			}
		}
	}
	for (i=0; i<nenvs; i++) {
		if (envs+i == lastenv) continue;
		for (j=0; j<envs[i].ntxns; j++) {
			if (envs[i].txns[j].ttxn == ttxn) {
				lastenv = envs+i;
				lasttxn = envs[i].txns+j;
				return lasttxn;
			}
		}
	}
	assert(0);	/* should have found it */
}

static void deltxn(txnpair *tp)
{
	int i = tp - lastenv->txns;
	for (; i<lastenv->ntxns-1; i++)
		lastenv->txns[i] = lastenv->txns[i+1];
	lastenv->ntxns--;
	lasttxn = NULL;
}

static void addcrs(txnpair *tp, void *tcrs, MDB_cursor *rcrs)
{
	int j = tp->ncursors;
	assert(tp->ncursors < MAXCRSS);

	tp->cursors[j].tcrs = tcrs;
	tp->cursors[j].rcrs = rcrs;
	tp->ncursors++;
	lastcrs = tp->cursors+j;
}

static crspair *findcrs(void *tcrs)
{
	int i, j, k;
	envpair *ep;
	txnpair *tp;
	crspair *cp;
	if (lastcrs && lastcrs->tcrs == tcrs)
		return lastcrs;
	if (lasttxn) {
		for (k=0, cp=lasttxn->cursors; k<lasttxn->ncursors; k++, cp++) {
			if (cp->tcrs == tcrs) {
				lastcrs = cp;
				return lastcrs;
			}
		}
	}
	if (lastenv) {
		for (j=0, tp=lastenv->txns; j<lastenv->ntxns; j++, tp++){
			if (tp == lasttxn)
				continue;
			for (k=0, cp = tp->cursors; k<tp->ncursors; k++, cp++) {
				if (cp->tcrs == tcrs) {
					lastcrs = cp;
					lasttxn = tp;
					return lastcrs;
				}
			}
		}
	}
	for (i=0, ep=envs; i<nenvs; i++, ep++) {
		for (j=0, tp=ep->txns; j<ep->ntxns; j++, tp++) {
			if (tp == lasttxn)
				continue;
			for (k=0, cp = tp->cursors; k<tp->ncursors; k++, cp++) {
				if (cp->tcrs == tcrs) {
					lastcrs = cp;
					lasttxn = tp;
					lastenv = ep;
					return lastcrs;
				}
			}
		}
	}
	assert(0);	/* should have found it already */
}

static void delcrs(void *tcrs)
{
	int i;
	crspair *cp = findcrs(tcrs);
	mdb_cursor_close(cp->rcrs);
	for (i = cp - lasttxn->cursors; i<lasttxn->ncursors-1; i++)
		lasttxn->cursors[i] = lasttxn->cursors[i+1];
	lasttxn->ncursors--;
	lastcrs = NULL;
}

void child()
{
	int rc;
	MDB_val key, data;
	char *ptr;

	while (fgets(inbuf, sizeof(inbuf), stdin)) {
		ptr = inbuf;
		if (!strncmp(ptr, SCMP("exit")))
			break;

		if (!strncmp(ptr, SCMP("mdb_env_create"))) {
			void *tenv;
			MDB_env *renv;
			sscanf(ptr+SOFF("mdb_env_create"), "%p", &tenv);
			E(mdb_env_create(&renv));
			addenv(tenv, renv);
		} else if (!strncmp(ptr, SCMP("mdb_env_set_maxdbs"))) {
			void *tenv;
			envpair *ep;
			unsigned int maxdbs;
			sscanf(ptr+SOFF("mdb_env_set_maxdbs"), "%p, %u", &tenv, &maxdbs);
			ep = findenv(tenv);
			E(mdb_env_set_maxdbs(ep->renv, maxdbs));
		} else if (!strncmp(ptr, SCMP("mdb_env_set_mapsize"))) {
			void *tenv;
			envpair *ep;
			mdb_size_t mapsize;
			sscanf(ptr+SOFF("mdb_env_set_mapsize"), "%p, %"MDB_SCNy(u), &tenv, &mapsize);
			ep = findenv(tenv);
			E(mdb_env_set_mapsize(ep->renv, mapsize));
		} else if (!strncmp(ptr, SCMP("mdb_env_open"))) {
			void *tenv;
			envpair *ep;
			char *path;
			int len;
			unsigned int flags, mode;
			sscanf(ptr+SOFF("mdb_env_open"), "%p, %n", &tenv, &len);
			path = ptr+SOFF("mdb_env_open")+len;
			ptr = strchr(path, ',');
			*ptr++ = '\0';
			sscanf(ptr, "%u, %o", &flags, &mode);
			ep = findenv(tenv);
			E(mdb_env_open(ep->renv, path, flags, mode));
			if (!maxkey) {
				maxkey = mdb_env_get_maxkeysize(ep->renv);
				kbuf = malloc(maxkey+2);
				dbuf = malloc(maxkey+2);
				dbufsize = maxkey;
			}
		} else if (!strncmp(ptr, SCMP("mdb_env_close"))) {
			void *tenv;
			envpair *ep;
			sscanf(ptr+SOFF("mdb_env_close"), "%p", &tenv);
			ep = findenv(tenv);
			mdb_env_close(ep->renv);
			delenv(ep);
			if (!nenvs)	/* if no other envs left, this process is done */
				break;
		} else if (!strncmp(ptr, SCMP("mdb_txn_begin"))) {
			unsigned int flags;
			void *tenv, *ttxn;
			envpair *ep;
			MDB_txn *rtxn;
			sscanf(ptr+SOFF("mdb_txn_begin"), "%p, %*p, %u = %p", &tenv, &flags, &ttxn);
			ep = findenv(tenv);
			E(mdb_txn_begin(ep->renv, NULL, flags, &rtxn));
			addtxn(tenv, ttxn, rtxn);
		} else if (!strncmp(ptr, SCMP("mdb_txn_commit"))) {
			void *ttxn;
			txnpair *tp;
			sscanf(ptr+SOFF("mdb_txn_commit"), "%p", &ttxn);
			tp = findtxn(ttxn);
			E(mdb_txn_commit(tp->rtxn));
			deltxn(tp);
		} else if (!strncmp(ptr, SCMP("mdb_txn_abort"))) {
			void *ttxn;
			txnpair *tp;
			sscanf(ptr+SOFF("mdb_txn_abort"), "%p", &ttxn);
			tp = findtxn(ttxn);
			mdb_txn_abort(tp->rtxn);
			deltxn(tp);
		} else if (!strncmp(ptr, SCMP("mdb_dbi_open"))) {
			void *ttxn;
			txnpair *tp;
			char *dbname;
			unsigned int flags;
			unsigned int tdbi;
			MDB_dbi dbi;
			sscanf(ptr+SOFF("mdb_dbi_open"), "%p, ", &ttxn);
			dbname = strchr(ptr+SOFF("mdb_dbi_open"), ',');
			dbname += 2;
			ptr = strchr(dbname, ',');
			*ptr++ = '\0';
			if (!strcmp(dbname, "(null)"))
				dbname = NULL;
			sscanf(ptr, "%u = %u", &flags, &tdbi);
			tp = findtxn(ttxn);
			E(mdb_dbi_open(tp->rtxn, dbname, flags, &dbi));
		} else if (!strncmp(ptr, SCMP("mdb_dbi_close"))) {
			void *tenv;
			envpair *ep;
			unsigned int tdbi;
			sscanf(ptr+SOFF("mdb_dbi_close"), "%p, %u", &tenv, &tdbi);
			ep = findenv(tenv);
			mdb_dbi_close(ep->renv, tdbi);
		} else if (!strncmp(ptr, SCMP("mdb_cursor_open"))) {
			void *ttxn, *tcrs;
			txnpair *tp;
			MDB_cursor *rcrs;
			unsigned int tdbi;
			sscanf(ptr+SOFF("mdb_cursor_open"), "%p, %u = %p", &ttxn, &tdbi, &tcrs);
			tp = findtxn(ttxn);
			E(mdb_cursor_open(tp->rtxn, tdbi, &rcrs));
			addcrs(tp, tcrs, rcrs);
		} else if (!strncmp(ptr, SCMP("mdb_cursor_close"))) {
			void *tcrs;
			sscanf(ptr+SOFF("mdb_cursor_close"), "%p", &tcrs);
			delcrs(tcrs);
		} else if (!strncmp(ptr, SCMP("mdb_cursor_put"))) {
			void *tcrs;
			crspair *cp;
			unsigned int flags;
			int len;
			sscanf(ptr+SOFF("mdb_cursor_put"), "%p, ", &tcrs);
			cp = findcrs(tcrs);
			ptr = strchr(ptr+SOFF("mdb_cursor_put"), ',');
			sscanf(ptr+1, "%"MDB_SCNy(u)",", &key.mv_size);
			if (key.mv_size) {
				ptr = strchr(ptr, '[');
				inhex(ptr+1, kbuf);
				key.mv_data = kbuf;
				ptr += key.mv_size * 2 + 2;
			}
			ptr = strchr(ptr+1, ',');
			sscanf(ptr+1, "%"MDB_SCNy(u)"%n", &data.mv_size, &len);
			if (data.mv_size > dbufsize) {
				dbuf = realloc(dbuf, data.mv_size+2);
				assert(dbuf != NULL);
				dbufsize = data.mv_size;
			}
			ptr += len+1;
			if (*ptr == '[') {
				inhex(ptr+1, dbuf);
				data.mv_data = dbuf;
				ptr += data.mv_size * 2 + 2;
			} else {
				sprintf(dbuf, "%09ld", (long)mdb_txn_id(lasttxn->rtxn));
			}
			sscanf(ptr+1, "%u", &flags);
			E(mdb_cursor_put(cp->rcrs, &key, &data, flags));
		} else if (!strncmp(ptr, SCMP("mdb_cursor_del"))) {
			void *tcrs;
			crspair *cp;
			unsigned int flags;
			sscanf(ptr+SOFF("mdb_cursor_del"), "%p, %u", &tcrs, &flags);
			cp = findcrs(tcrs);
			E(mdb_cursor_del(cp->rcrs, flags));
		} else if (!strncmp(ptr, SCMP("mdb_put"))) {
			void *ttxn;
			txnpair *tp;
			unsigned int tdbi, flags;
			int len;
			sscanf(ptr+SOFF("mdb_put"),"%p, %u, %"MDB_SCNy(u), &ttxn, &tdbi, &key.mv_size);
			tp = findtxn(ttxn);
			ptr = strchr(ptr+SOFF("mdb_put"), '[');
			inhex(ptr+1, kbuf);
			key.mv_data = kbuf;
			ptr += key.mv_size * 2 + 2;
			sscanf(ptr+1, "%"MDB_SCNy(u)"%n", &data.mv_size, &len);
			if (data.mv_size > dbufsize) {
				dbuf = realloc(dbuf, data.mv_size+2);
				assert(dbuf != NULL);
				dbufsize = data.mv_size;
			}
			ptr += len+1;
			if (*ptr == '[') {
				inhex(ptr+1, dbuf);
				ptr += data.mv_size * 2 + 2;
			} else {
				sprintf(dbuf, "%09ld", (long)mdb_txn_id(tp->rtxn));
			}
			data.mv_data = dbuf;
			sscanf(ptr+1, "%u", &flags);
			RES(MDB_KEYEXIST,mdb_put(tp->rtxn, tdbi, &key, &data, flags));
		} else if (!strncmp(ptr, SCMP("mdb_del"))) {
			void *ttxn;
			txnpair *tp;
			unsigned int tdbi;
			int len;
			sscanf(ptr+SOFF("mdb_del"),"%p, %u, %"MDB_SCNy(u), &ttxn, &tdbi, &key.mv_size);
			tp = findtxn(ttxn);
			ptr = strchr(ptr+SOFF("mdb_del"), '[');
			inhex(ptr+1, kbuf);
			key.mv_data = kbuf;
			ptr += key.mv_size * 2 + 2;
			sscanf(ptr+1, "%"MDB_SCNy(u)"%n", &data.mv_size, &len);
			if (data.mv_size > dbufsize) {
				dbuf = realloc(dbuf, data.mv_size+2);
				assert(dbuf != NULL);
				dbufsize = data.mv_size;
			}
			ptr += len+1;
			if (*ptr == '[') {
				inhex(ptr+1, dbuf);
			} else {
				sprintf(dbuf, "%09ld", (long)mdb_txn_id(tp->rtxn));
			}
			data.mv_data = dbuf;
			RES(MDB_NOTFOUND,mdb_del(tp->rtxn, tdbi, &key, &data));
		}
		write(1, "\n", 1);
	}
	exit(0);
}

static pidpair *addpid(int tpid)
{
	int fdout[2], fdin[2];
	pid_t pid;
	assert(npids < MAXPIDS);
	pids[npids].tpid = tpid;
	pipe(fdout);
	pipe(fdin);
	if ((pid = fork()) == 0) {
		/* child */
		fclose(stdin);
		fclose(stdout);
		dup2(fdout[0], 0);
		dup2(fdin[1], 1);
		stdin = fdopen(0, "r");
		stdout = fdopen(1, "w");
		child();
		return 0;	/* NOTREACHED */
	} else {
		pids[npids].rpid = pid;
		pids[npids].fdout = fdout[1];
		pids[npids].fdin = fdin[0];
		lastpid = pids+npids;
		npids++;
		return lastpid;
	}
}

static pidpair *findpid(int tpid)
{
	int i;
	if (!lastpid || lastpid->tpid != tpid) {
		for (i=0; i<npids; i++)
			if (pids[i].tpid == tpid)
				break;
		if (i == npids)
			return NULL;
		lastpid = &pids[i];
	}
	return lastpid;
}

volatile pid_t killpid;

static void delpid(int tpid)
{
	pidpair *pp = findpid(tpid);
	if (pp) {
		pid_t kpid = pp->rpid;
		killpid = kpid;
		write(pp->fdout, "exit\n", sizeof("exit"));
		while (killpid == kpid)
			usleep(10000);
	}
}

static void reaper(int sig)
{
	int status, i;
	pid_t pid = waitpid(-1, &status, 0);
	if (pid > 0) {
		fprintf(stderr, "# %s %d\n", WIFEXITED(status) ? "exited" : "killed", pid);
		for (i=0; i<npids; i++)
			if (pids[i].rpid == pid)
				break;
		assert(i < npids);
		close(pids[i].fdout);
		close(pids[i].fdin);
		for (;i<npids-1; i++)
			pids[i] = pids[i+1];
		npids--;
		killpid = 0;
	}
}

int main(int argc,char * argv[])
{
	signal(SIGCHLD, reaper);

	while (fgets(inbuf, sizeof(inbuf), stdin)) {
		pidpair *pp;
		int tpid, len;
		char c, *ptr;
		lcount++;

		if (inbuf[0] == '#' && !strncmp(inbuf+1, SCMP(" killed"))) {
			sscanf(inbuf+SOFF("killed"),"%d", &tpid);
			delpid(tpid);
			continue;
		}

		if (inbuf[0] != '>')
			continue;
		ptr = inbuf+1;
		sscanf(ptr, "%d:%n", &tpid, &len);
		pp = findpid(tpid);
		if (!pp)
			pp = addpid(tpid);	/* new process */

		ptr = inbuf+len+1;
		len = strlen(ptr);
		write(pp->fdout, ptr, len);	/* send command and wait for ack */
		read(pp->fdin, &c, 1);
	}
	while (npids)
		delpid(pids[0].tpid);
}
